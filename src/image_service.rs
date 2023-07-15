use std::io::Error;

use serde::{Deserialize, Serialize};

/**
 * Service for creating cached/optimized images!
 */

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub struct CachedImage {
    pub src: String,
    pub option: CachedImageOption,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub enum CachedImageOption {
    Resize(Resize),
    Blur(Blur),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub struct Resize {
    pub width: u32,
    pub height: u32,
    pub quality: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub struct Blur {
    pub width: u32,
    pub height: u32,
    pub svg_width: u32,
    pub svg_height: u32,
    pub sigma: u8,
}

#[test]
fn test_url_encode() {
    let img = CachedImage {
        src: "test.jpg".to_string(),
        option: CachedImageOption::Resize(Resize {
            quality: 75,
            width: 100,
            height: 100,
        }),
    };

    let encoded = img.get_url_encoded();
    let decoded: CachedImage = CachedImage::from_url_encoded(&encoded);

    dbg!(encoded);
    assert!(img == decoded);
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub enum CreateImageError {
    ImageError(image::ImageError),
    JoinError(tokio::task::JoinError),
    IOError(Error),
}

impl CachedImage {
    pub fn get_file_path(&self) -> String {
        // todo: ensure that src has leading slash.
        let path = match self.option {
            CachedImageOption::Resize(Resize {
                quality,
                width,
                height,
            }) => {
                format!("cache/resize/q{quality}_w{width}_h{height}",)
            }
            CachedImageOption::Blur(Blur {
                height,
                width,
                svg_height,
                svg_width,
                sigma,
            }) => {
                format!("cache/blur/w{width}_h{height}_sh{svg_height}_sw{svg_width}_s{sigma}")
            }
        };

        let mut path = path_from_segments(vec![&path, &self.src]);

        if let CachedImageOption::Resize { .. } = self.option {
            path.set_extension("webp");
        } else {
            path.set_extension("svg");
        };

        path.as_path().to_string_lossy().to_string()
    }

    pub fn get_file_path_from_root(&self, root: &str) -> String {
        let path = path_from_segments(vec![root, &self.get_file_path()]);
        path.as_path().to_string_lossy().to_string()
    }

    pub fn get_url_encoded(&self) -> String {
        // TODO: make this configurable?
        let image_cache_path = "/cache/image";
        let params = serde_qs::to_string(&self).unwrap();
        format!("{}?{}", image_cache_path, params)
    }

    pub fn from_url_encoded(url: &str) -> CachedImage {
        let url = url.split("?").filter(|s| *s != "?").last().unwrap_or(url);
        let result: Result<CachedImage, serde_qs::Error> = serde_qs::from_str(&url);
        result.expect("Failed to Cache Image Url")
    }

    // Returns the relative path as a string of the created image from the root.
    #[cfg(feature = "ssr")]
    pub async fn create_image(&self, root: &str) -> Result<String, CreateImageError> {
        let relative_path_created = self.get_file_path();

        let save_path = path_from_segments(vec![root, &relative_path_created]);
        let absolute_src_path = path_from_segments(vec![root, &self.src]);

        if file_exists(&save_path).await {
            Ok(relative_path_created)
        } else {
            let task = tokio::task::spawn_blocking({
                let config = self.clone();
                move || cache_image(config.option, absolute_src_path, save_path)
            });

            match task.await {
                Err(join_error) => Err(CreateImageError::JoinError(join_error)),
                Ok(Err(err)) => Err(err),
                Ok(Ok(_)) => Ok(relative_path_created),
            }
        }
    }
}

#[cfg(feature = "ssr")]
fn cache_image<P>(
    config: CachedImageOption,
    source_path: P,
    save_path: P,
) -> Result<(), CreateImageError>
where
    P: AsRef<std::path::Path> + AsRef<std::ffi::OsStr>,
{
    use webp::*;

    match config {
        CachedImageOption::Resize(Resize {
            width,
            height,
            quality,
        }) => {
            let img = image::open(source_path).map_err(|e| CreateImageError::ImageError(e))?;
            let new_img = img.resize(
                width,
                height,
                // Cubic Filter.
                image::imageops::FilterType::CatmullRom,
            );
            // Create the WebP encoder for the above image
            let encoder: Encoder = Encoder::from_image(&new_img).unwrap();
            // Encode the image at a specified quality 0-100
            let webp: WebPMemory = encoder.encode(quality as f32);
            create_nested_if_needed(&save_path).map_err(|e| CreateImageError::IOError(e))?;
            std::fs::write(save_path, &*webp).map_err(|e| CreateImageError::IOError(e))
        }
        CachedImageOption::Blur(blur) => {
            let svg = encode_blur(source_path, blur)?;
            create_nested_if_needed(&save_path).map_err(|e| CreateImageError::IOError(e))?;
            std::fs::write(save_path, &*svg).map_err(|e| CreateImageError::IOError(e))
        }
    }
}

#[cfg(feature = "ssr")]
fn encode_blur<P>(source_path: P, blur: Blur) -> Result<String, CreateImageError>
where
    P: AsRef<std::path::Path> + AsRef<std::ffi::OsStr>,
{
    use webp::*;

    let start = std::time::Instant::now();

    let img = image::open(source_path).map_err(|e| CreateImageError::ImageError(e))?;

    let Blur {
        width,
        height,
        svg_height,
        svg_width,
        sigma,
    } = blur;

    let img = img.resize(width, height, image::imageops::FilterType::Nearest);

    let resize_time = std::time::Instant::now();

    let elapsed = resize_time.duration_since(start);
    println!("resized image in {}", elapsed.as_millis());

    // Create the WebP encoder for the above image
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    // Encode the image at a specified quality 0-100
    let webp: WebPMemory = encoder.encode(80.0);

    let webp_time = std::time::Instant::now();
    let elapsed = webp_time.duration_since(resize_time);
    println!("converted to webp in {}", elapsed.as_millis());

    use base64::{engine::general_purpose, Engine as _};
    let encoded = general_purpose::STANDARD.encode(&*webp);

    let uri = format!("data:image/webp;base64,{}", encoded);

    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="100%" height="100%" viewBox="0 0 {svg_width} {svg_height}" preserveAspectRatio="none">
            <filter id="a" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB"> 
                <feGaussianBlur stdDeviation="{sigma}" edgeMode="duplicate"/> 
                <feComponentTransfer>
                    <feFuncA type="discrete" tableValues="1 1"/> 
                </feComponentTransfer>
            </filter> 
            <image filter="url(#a)" x="0" y="0" height="100%" width="100%" href="{uri}"/>
         </svg>
         "#,
    );

    // let svg_encoded = general_purpose::STANDARD.encode(svg.as_bytes());

    // let style= format!(
    //     "color:transparent;max-width:100%;height:auto;background-size:cover;background-position:50% 50%;background-repeat:no-repeat;background-image: url('data:image/svg+xml;base64,{}');",
    //     svg_encoded
    // );

    Ok(svg)
}

#[test]
fn test_encode() {
    let result = encode_blur(
        "test.jpg",
        Blur {
            width: 25,
            height: 25,
            svg_height: 100,
            svg_width: 100,
            sigma: 20,
        },
    );
    println!("{}", result.unwrap());
}

// #[cfg(feature = "ssr")]
fn path_from_segments(segments: Vec<&str>) -> std::path::PathBuf {
    segments
        .into_iter()
        .map(|s| s.trim_start_matches('/'))
        .map(|s| s.trim_end_matches('/'))
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(feature = "ssr")]
async fn file_exists<P>(path: P) -> bool
where
    P: AsRef<std::path::Path>,
{
    tokio::fs::metadata(path).await.is_ok()
}

#[cfg(feature = "ssr")]
fn create_nested_if_needed<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<std::ffi::OsStr>,
{
    match std::path::Path::new(&path).parent() {
        Some(p) if (!(p).exists()) => std::fs::create_dir_all(p),
        Some(_) => Result::Ok(()),
        None => Result::Ok(()),
    }
}

#[cfg(feature = "ssr")]
pub async fn cache_app_images<IV>(
    root: String,
    app_fn: impl Fn(leptos::Scope) -> IV + 'static,
) -> Result<(), crate::optimizer::CreateImageError>
where
    IV: leptos::IntoView + 'static,
{
    use crate::optimizer::CreateImageError;

    let images = crate::introspect::find_app_images(app_fn);
    let all_images: Vec<_> = images
        .clone()
        .into_iter()
        .map(|img| {
            let root = root.clone();
            tokio::spawn(async move { img.create_image(&root).await })
        })
        .collect();

    log::info!("Creating {} cached images", &all_images.len());

    let result: Result<Vec<_>, CreateImageError> = futures::future::join_all(all_images)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    let _ = result?;

    let image_data = images
        .into_iter()
        .filter_map(|img| match img.option {
            crate::optimizer::CachedImageOption::Blur(_) => {
                let path = img.get_file_path_from_root(&root);
                Some((img, path))
            }
            _ => None,
        })
        // Read all svg files into memory.
        .filter_map(|(img, path)| std::fs::read_to_string(path).ok().map(|svg| (img, svg)))
        .collect::<Vec<_>>();

    crate::provider::add_image_cache(image_data);
    Ok(())
}

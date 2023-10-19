// use std::net::SocketAddr;
// use std::path::PathBuf;

// use actix_files::NamedFile;
// use cfg_if::cfg_if;
// use http::Uri;
// use leptos::leptos_config::Env;

// use crate::app::App;
// use actix_web::http::StatusCode;
// use actix_web::HttpRequest as ActixRequest;
// use actix_web::HttpResponse as ActixResponse;
// use leptos::*;

// // pub struct LeptosOptions {
// //     pub output_name: String,
// //     pub site_root: String,
// //     pub site_pkg_dir: String,
// //     pub env: Env,
// //     pub site_addr: SocketAddr,
// //     pub reload_port: u32,
// //     pub reload_external_port: Option<u32>,
// //     pub reload_ws_protocol: ReloadWSProtocol,
// //     pub not_found_path: String,
// // }

// cfg_if! { if #[cfg(feature = "ssr")] {

//     pub async fn file_and_error_handler(uri: Uri, root:String, req: ActixRequest) -> ActixResponse {

//         let res = get_static_file(uri.clone(), &root, req).await.unwrap();

//         if res.status() == StatusCode::OK {
//             res
//         } else {

//             let handler = leptos_actix::render_app_to_stream(options.to_owned(), move || view!{<App/>}, leptos_router::Method::Get);
//             handler(req).await.into_response()
//         }
//     }

//     async fn get_static_file(uri: Uri, root: &str, req: ActixRequest) -> Result<ActixResponse, (StatusCode, String)> {

//         let file_path = PathBuf::from(format!("{}/{}", root, uri));
//         let named_file = NamedFile::open(file_path);

//         match named_file {
//             Ok(file) => {
//                 let response = file.into_response(&req);
//                 Ok(response)
//             }
//             Err(e) => {
//                 log::error!("Failed to create image: {:?}", e);
//                 Err((StatusCode::NOT_FOUND, "Error creating image.".to_string()))
//             }
//         }

//     }
// }}

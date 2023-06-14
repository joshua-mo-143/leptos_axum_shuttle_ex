use axum::{routing::get, Router};
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "static")] public: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .nest_service(
            "/",
            ServeDir::new(&public).not_found_service(ServeFile::new(public.join("/index.html"))),
        );

    Ok(router.into())
}

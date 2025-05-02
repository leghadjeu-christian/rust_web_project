use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

use datbase::query_status;
use endpoints::{compress_status::{self, get_status}, file_upload_endpoint};
use file_upload_endpoint::upload;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

use rust_hello_server::compression;

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {


    let url = "http://localhost:8000/";

    let uploads = Router::new()
        .route("/upload", post(upload))
        .nest_service("/files", ServeDir::new("uploads"));

    let compress = Router::new().route("/status/:id", get(get_status));

 let router = Router::new()
        .route("/", get(hello_world))
        .nest("/uploads", uploads)
        .nest("/compress", compress);

    println!("[URL] {}", url);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}
mod compress;
mod datbase;
mod endpoints;

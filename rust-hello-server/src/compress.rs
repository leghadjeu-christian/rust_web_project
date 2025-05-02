use std::path::PathBuf;

use crate::compression;
use crate::datbase::{get_file_ref, update_db};
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

//// Endpoint to trigger file compression on demand

pub async fn compress_files(pool: Pool<Postgres>, id: i32) -> impl IntoResponse {
    let file_ref = get_file_ref(pool.clone(), id).await;

    let input = PathBuf::from(file_ref);

    // let uploads_dir = PathBuf::from("uploads");
    let compressed_dir = PathBuf::from("compressed");

    // Ensure the compressed directory exists
    if let Err(e) = std::fs::create_dir_all(&compressed_dir) {
        return format!("Failed to create compressed directory: {}", e);
    }

    let input_path = &input;
    let output_path = compressed_dir.join(input.as_path()).with_extension("gz");

    // Compress the file and handle errors
    if let Err(e) = compression(
        input_path.to_str().to_owned().unwrap().to_string(),
        output_path.to_str().to_owned().unwrap().to_string(),
    ) {
        let update_db2 = update_db(pool.clone(), "compression_failed", id);
        let _ = tokio::join!(update_db2);
        return format!("Failed to compress file {}: {}", input_path.display(), e);
    }
    let update_db = update_db(pool.clone(), "compressed", id);
    let _ = tokio::join!(update_db);

    String::from("Files compressed successfully")
}

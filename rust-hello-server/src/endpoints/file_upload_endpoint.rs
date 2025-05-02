use crate::{compress::compress_files, datbase::insert_file};
use crate::datbase::query_result;
use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mysecretpassword@localhost:5432/filesdb")
        .await
    {
        Ok(p) => p,
        Err(_) => {
            panic!("Error cnnectiong to database");
        }
    };

    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        // if field.name().unwrap() != "fileupload" {
        //     continue;
        // }
        println!("Got file");

        // Grab the name
        let file_name = field.file_name().unwrap();

        // Create a path for the soon-to-be file
        let file_path = format!("{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();
        let path = PathBuf::new();
        let new_name = path.join("uploads").join(file_path.clone());
        // println!("the path is : {:?}", new_name);
        println!("file path : {}", file_path);

        // Open a handle to the file
        let mut file_handle = File::create(new_name).expect("Failed to create file handle");

        // Write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");

        let id = insert_file(pool.clone(), &file_path).await.expect("failed to insert to database");
        println!("the id is : {}", id);

        let _res = compress_files(pool.clone(), id);
        tokio::join!(_res);
        // println!("The compress response :{:?}", res);
    }

    String::from(format!(
        r#"{{"status": 201,"message": "File uploaded successfully.","files""#,
    ))
}

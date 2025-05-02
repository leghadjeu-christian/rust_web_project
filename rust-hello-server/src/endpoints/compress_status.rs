use crate::datbase::query_status;
use axum::{extract::Path, response::IntoResponse};
use sqlx::postgres::PgPoolOptions;

pub async fn get_status(Path(parameter): Path<String>) -> impl IntoResponse {
    let id = parameter.parse().expect("Failed to parse string to i32");
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
    match  query_status(pool, id).await {
        Ok(status) => return String::from(format!(
            r#"{{"status": 200,"message": "File has a compressed status of : {:?}"#,
        status)),
        Err(e) => e,
    };
    String::from(format!(
        r#"{{"status": 200,"message": "File has a compressed status of : "#,
    ))
}

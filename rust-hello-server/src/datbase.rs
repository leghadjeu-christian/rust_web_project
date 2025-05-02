use crate::compression;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "status_enum")] // Ensure this matches the PostgreSQL type
pub enum StatusEnum {
    uploaded,
    compressed,
    compressing,
    compression_failed,
}

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres, Row};
#[inline(always)]

pub async fn insert_file(pool: Pool<Postgres>, file_ref: &str) -> Result<i32, sqlx::Error> {
    // Efficient database operation inside an async function
    let query = "INSERT INTO files (file_ref,   compressed_file_ref) VALUES ($1, $2)";

    // Use the query and await the result asynchronously
    let row = sqlx::query(query)
        .bind(file_ref)
        .bind("ref")
        .execute(&pool)
        .await?; // Return error if there's an issue with the query
    let query_result = select(pool, file_ref).await.expect("Failed to query DB");

    let id: i32 = query_result.get("id");

    Ok(id)
}

pub async fn query_result(pool: Pool<Postgres>, id: i32) -> Result<PgRow, sqlx::Error> {
    let query = format!("SELECT * FROM files WHERE id = {}", id);
    let query_result = sqlx::query(&query).fetch_one(&pool).await?;
    Ok(query_result)
}
pub async fn select(pool: Pool<Postgres>, file_ref: &str) -> Result<PgRow, sqlx::Error> {
    let query = format!("SELECT * FROM files WHERE file_ref = $1");
    let query_result = sqlx::query(&query).bind(file_ref).fetch_one(&pool).await?;
    Ok(query_result)
}

pub async fn get_file_ref(pool: Pool<Postgres>, id: i32) -> String {
    let query_result = query_result(pool, id)
        .await
        .expect("failed to get file_ref");
    let file_ref: String = query_result.get("file_ref");
    file_ref
}

pub async fn update_db(pool: Pool<Postgres>, update_to: &str, id: i32) -> Result<(), sqlx::Error> {
    match update_to {
        "compressing" => {
            let query = "UPDATE files SET status = $1 WHERE id = $2";
            let _query_result = sqlx::query(&query)
                .bind(StatusEnum::compressing)
                .bind(id)
                .execute(&pool)
                .await?;
            Ok(())
        }

        "compressed" => {
            let query = "UPDATE files SET status = $1 WHERE id = $2";
            let _query_result = sqlx::query(&query)
                .bind(StatusEnum::compressed)
                .bind(id)
                .execute(&pool)
                .await?;
            Ok(())
        }

        "compression_failed" => {
            let query = "UPDATE files SET status = $1 WHERE id = $2";
            let _query_result = sqlx::query(&query)
                .bind(StatusEnum::compression_failed)
                .bind(id)
                .execute(&pool)
                .await?;
            Ok(())
        }
        _ => Ok(()),
    }
}

pub async fn query_status(pool: Pool<Postgres>, id: i32) -> Result<StatusEnum, sqlx::Error> {
    let query = "SELECT status FROM files WHERE id = $1";
    match sqlx::query(query).bind(id).fetch_one(&pool).await {
        Ok(row) => {
            let status: StatusEnum = row.get("id");
            return Ok(status);
        }
        Err(e) => Err(e),
    }
}


union MyUnion {
    x: i32,
    y: f32,
}
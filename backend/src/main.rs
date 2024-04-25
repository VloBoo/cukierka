use crate::database::Database;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::Filter;

mod api;
mod apitool;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let lock_db = Arc::new(Mutex::new(Database::new().await?));

    let lock_db_clone1 = lock_db.clone();
    let sql = warp::path!("api" / "user" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = lock_db_clone1.clone();
            async move { api::get_user(id, cloned).await }
        });

    let lock_db_clone2 = lock_db.clone();
    let sql2 = warp::path!("api" / "user")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = lock_db_clone2.clone();
            async move { api::create_user(req_json, cloned).await }
        });

    warp::serve(sql2.or(sql))
        //.tls()
        // .cert_path("secret/cert.crt")
        // .key_path("secret/key.rsa")
        .run(([0, 0, 0, 0], 8081))
        .await;

    return Ok(());
}

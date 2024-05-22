use crate::database::Database;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

mod api;
mod database;
mod route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let lock_db = Arc::new(Mutex::new(
        Database::new()
            .await
            .inspect_err(|e| log::error!("{:?}", e))?,
    ));

    warp::serve(route::route(lock_db))
        //.tls()
        // .cert_path("secret/cert.crt")
        // .key_path("secret/key.rsa")
        .run(([0, 0, 0, 0], 4444))
        .await;

    return Ok(());
}

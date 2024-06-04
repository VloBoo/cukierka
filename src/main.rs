use config::*;
use sqlx::PgPool;

mod api;
mod config;
mod database;
mod error;
mod route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let db = PgPool::connect(DATABASE_URL).await?;

    let listener = tokio::net::TcpListener::bind(IP_ADDR)
        .await
        .unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, route::route(db)).await.unwrap();

    /*
     let addr = SocketAddr::from((IpAddr::from_str("::0").unwrap(), 4444));
    let lock_db = Arc::new(Mutex::new(
        Database::new()
            .await
            .inspect_err(|e| log::error!("{:?}", e))?,
    ));

    warp::serve(route::route(lock_db))
        //.tls()
        // .cert_path("secret/cert.crt")
        // .key_path("secret/key.rsa")
        .run(addr)
        .await;
    */
    return Ok(());
}

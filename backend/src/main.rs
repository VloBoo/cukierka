use serde_json::json;
use std::collections::HashMap;
use warp::Filter;

use crate::database::Database;

mod database;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("{:?}", Database::hardsql("SELECT 1 + 1 as sum").await);

    let sql_proxy = warp::path("sql").and(warp::body::json()).and_then(
        |simple_map: HashMap<String, String>| async move {
            let sql = match simple_map.get("sql") {
                Some(sql) => sql,
                None => {
                    log::error!("Строка запроса не найдена \n {:?}", simple_map);
                    panic!("Строка запроса не найдена!");
                }
            };

            log::info!("{}", sql);

            let result = Database::hardsql(sql).await.unwrap();

            Ok::<_, warp::Rejection>(warp::reply::json(&result))
        },
    );

    let api_also = warp::any().and_then(|| async move {
        Ok::<_, warp::Rejection>(warp::reply::json(&json!({"error":"404. Not Found."})))
    });

    let final_warp =
        warp::any().and(warp::post().and(warp::path("api").and(sql_proxy).or(api_also)));

    warp::serve(final_warp)
        //.tls()
        // .cert_path("secret/cert.crt")
        // .key_path("secret/key.rsa")
        .run(([0, 0, 0, 0], 80))
        .await;
}

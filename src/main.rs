use chrono::Utc;
use serde_json::{json, Value};
use std::{collections::HashMap, usize};
use tokio_postgres::NoTls;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let sql_proxy = warp::path("sql").and(warp::body::json()).and_then(
        |simple_map: HashMap<String, String>| async move {
            let (client, connection) =
                tokio_postgres::connect("host=localhost user=VloBo dbname=db", NoTls)
                    .await
                    .inspect_err(|e| {
                        log::error!("Не удалось подключиться к базе данных \n {:?}", e)
                    })
                    .expect("Не удалось подключиться к базе данных");
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    log::error!("connection error: {}", e);
                }
            });
            let sql = match simple_map.get("sql") {
                Some(sql) => sql,
                None => {
                    log::error!("Строка запроса не найдена \n {:?}", simple_map);
                    panic!("Строка запроса не найдена!");
                }
            };

            log::info!("{}", sql);

            let rows = match client.query(sql, &[]).await {
                Ok(value) => {
                    log::debug!("{:?}", value);
                    value
                }
                Err(error) => {
                    log::error!("Не удалось отправить запрос \n {:?}", error);
                    return Ok::<_, warp::Rejection>(warp::reply::json(
                        &json!({"error":"Not execute."}),
                    ));
                }
            };

            let mut rows_json = Vec::new();

            for row in &rows {
                let mut row_json = serde_json::Map::new();

                for (i, column) in row.columns().iter().enumerate() {
                    let column_name = column.name();
                    let value = if let Ok(value) = row.try_get::<usize, String>(i) {
                        json!(value)
                    } else if let Ok(value) = row.try_get::<usize, Uuid>(i) {
                        json!(value.to_string())
                    } else if let Ok(value) = row.try_get::<usize, chrono::DateTime<Utc>>(i) {
                        json!(value.to_rfc3339())
                    } else if let Ok(value) = row.try_get::<usize, i32>(i) {
                        json!(value)
                    } else if let Ok(value) = row.try_get::<usize, i8>(i) {
                        json!(value)
                    } else if let Ok(value) = row.try_get::<usize, i64>(i) {
                        json!(value)
                    } else if let Ok(value) = row.try_get::<usize, Vec<String>>(i) {
                        json!(value)
                    } else if let Ok(value) = row.try_get::<usize, Value>(i) {
                        json!(value)
                    } else {
                        Value::Null
                    };
                    row_json.insert(column_name.into(), value);
                }

                rows_json.push(Value::Object(row_json));
            }
            let result = json!({
                "rows": rows_json
            });
            log::debug!("{:?}", result);
            Ok::<_, warp::Rejection>(warp::reply::json(&result))
        },
    );

    let mail_sender = warp::path("email")
        .and_then(|| async move { Ok::<_, warp::Rejection>(warp::reply::html("No")) });

    let dir = warp::fs::dir("./www/");

    let api_also = warp::any().and_then(|| async move {
        Ok::<_, warp::Rejection>(warp::reply::json(&json!({"error":"404. Not Found."})))
    });

    let not_found = warp::any().map(|| {
        warp::reply::with_status(
            warp::reply::html("<h1 align=\"center\">404. Not Found.</h1>"),
            StatusCode::NOT_FOUND,
        )
    });

    let final_warp = warp::any()
        .and(
            warp::post().and(
                warp::path("api")
                    .and(sql_proxy.or(mail_sender))
                    .or(api_also),
            ),
        )
        .or(dir.or(not_found));

    warp::serve(final_warp)
        //.tls()
        // .cert_path("secret/cert.crt")
        // .key_path("secret/key.rsa")
        .run(([0, 0, 0, 0], 80))
        .await;
}

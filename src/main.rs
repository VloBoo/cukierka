use log::{error, info};
use uuid::{uuid, Uuid};
use warp::Filter;
use log4rs;

mod core;
mod db;
    
#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap_or_else(|e| {
        eprintln!("Failed to initialize log4rs: {}", e);
        std::process::exit(1);
    });

    info!("Info test");
    error!("Error test");
    
    let get_tables_route =
        warp::path("e")
            .and(warp::path::param())
            .and_then(|param: String| async move {
                match Uuid::parse_str(&param) {
                    Ok(r) => match db::get_user(&r).await {
                        Ok(result) => {
                            info!("{}", result);
                            return Ok::<_, warp::Rejection>(warp::reply::json(&result));
                        }
                        Err(err) => {
                            error!("Error: {:?}", err);
                            return Ok::<_, warp::Rejection>(warp::reply::json(
                                &"Error".to_string(),
                            ));
                        }
                    },
                    Err(_) => {
                        return Ok::<_, warp::Rejection>(warp::reply::json(&"Error".to_string()))
                    }
                }
            });

    let _get_tables_route2 = warp::path!("w")
        .and_then(|| async { Ok::<_, warp::Rejection>(warp::reply::html("Hello".to_string())) });

    let _hi = warp::path("q")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    let _route = warp::path("q").and(
        warp::path("qq")
            .map(|| "qq")
            .or(warp::path("qqq").map(|| "qqq")),
    );

    warp::serve(get_tables_route.or(_get_tables_route2).or(_hi))
        .run(([0, 0, 0, 0], 80))
        .await;
}

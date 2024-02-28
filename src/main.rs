use uuid::Uuid;
use warp::Filter;

mod core;
mod db;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::trace!("Trace test");
    log::debug!("Debug test");
    log::info!("Info test");
    log::warn!("Warn test");
    log::error!("Error test");

    let get_tables_route =
        warp::path("e")
            .and(warp::path::param())
            .and_then(|param: String| async move {
                log::info!("pls");
                match Uuid::parse_str(&param) {
                    Ok(r) => match db::get_user(&r).await {
                        Ok(result) => {
                            log::info!("{}", result);
                            return Ok::<_, warp::Rejection>(warp::reply::json(&result));
                        }
                        Err(err) => {
                            log::error!("Error: {:?}", err);
                            return Ok::<_, warp::Rejection>(warp::reply::json(
                                &"Error".to_string(),
                            ));
                        }
                    },
                    Err(_) => {
                        log::error!("no");
                        return Ok::<_, warp::Rejection>(warp::reply::json(&"Error".to_string()));
                    }
                }
            });

    /*
    let _hi = warp::path("q")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));
    */

    warp::serve(get_tables_route)
        .run(([0, 0, 0, 0], 80))
        .await;
}

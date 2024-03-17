use uuid::Uuid;
use warp::Filter;

mod api;
mod db;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    log::trace!("Trace test");
    log::debug!("Debug test");
    log::info!("Info test");
    log::warn!("Warn test");
    log::error!("Error test");

    let get_tables_route = warp::path("api")
        //.and(warp::path("e"))
        .and(warp::path::param())
        .and_then(|param: Uuid| async move {
            log::debug!("test api");
            Ok::<_, warp::Rejection>(warp::reply::json(&api::base::get_user_by_id(&param).await))
        });

    let test = warp::path("hi")
        .and_then(|| async move { Ok::<_, warp::Rejection>(warp::reply::html("hello")) });

    /*
    let _hi = warp::path("q")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));
    */

    warp::serve(get_tables_route.or(test))
        .tls()
        .cert_path("secret/cert.crt")
        .key_path("secret/key.rsa")
        .run(([0, 0, 0, 0], 443))
        .await;
}

use uuid::uuid;
use warp::Filter;

mod core;
mod db;

#[tokio::main]
async fn main() {

    let get_tables_route = warp::path!("e").and_then(|| async {
        let id = uuid!("78767867-e547-33e8-998b-3816d57e3e56");
        match db::get_user(&id).await {
            Ok(result) => {
                eprintln!("{}", result);
                return Ok::<_, warp::Rejection>(warp::reply::json(&result));
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return Ok::<_, warp::Rejection>(warp::reply::json(&"Error".to_string()));
            }
        };
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

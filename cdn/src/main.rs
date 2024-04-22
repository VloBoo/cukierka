use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use warp::http::{Response, StatusCode};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let upload_route = warp::path("cdn")
        .and(warp::post())
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .and_then(upload_handler);

    let download_route = warp::path("cdn")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and_then(download_handler);

    let routes = upload_route.or(download_route);

    warp::serve(routes).run(([0, 0, 0, 0], 5550)).await;
}

async fn upload_handler(
    filename: String,
    payload: warp::hyper::body::Bytes,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(filetype) = filename.split(".").last() {
        let new_filename = format!("{}.{}", uuid::Uuid::new_v4().to_string(), filetype);
        let filepath = format!("/var/content/{}", new_filename);

        let mut file = File::create(filepath.clone())
            .await
            .map_err(|_| warp::reject())?;
        file.write_all(&payload).await.map_err(|_| warp::reject())?;

        return Ok(Response::builder()
            .status(StatusCode::CREATED)
            .header("Location", format!("/cdn/{}", new_filename))
            .body("Created")
            .unwrap());
    } else {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found")
            .unwrap());
    }
}

async fn download_handler(filename: String) -> Result<impl warp::Reply, warp::Rejection> {
    let filepath = format!("/var/content/{}", filename);
    let path = PathBuf::from(filepath);

    if let Ok(mut file) = File::open(path).await {
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .await
            .map_err(|_| warp::reject())?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Cache-Control", "public, max-age=3600")
            .header("Content-Type", "application/octet-stream")
            .body(content)
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("File not found".into())
            .unwrap())
    }
}

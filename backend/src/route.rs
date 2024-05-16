use crate::{api, database::Database};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::Filter;

pub(crate) fn route(
    lock_db: Arc<Mutex<Database>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let warp_hardsql = warp::path!("api" / "sql").and(warp::body::json()).and_then(
        |json: HashMap<String, String>| async move {
            Ok::<_, warp::Rejection>(warp::reply::json(
                &Database::hardsql(json.get("sql").unwrap()).await.unwrap(),
            ))
        },
    );

    let db_clone_get_user = lock_db.clone();
    let warp_get_user = warp::path!("api" / "user" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_user.clone();
            async move { api::user::get(id, cloned).await }
        });

    let db_clone_create_user = lock_db.clone();
    let warp_create_user = warp::path!("api" / "user")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = db_clone_create_user.clone();
            async move { api::user::create(req_json, cloned).await }
        });

    let db_clone_get_token = lock_db.clone();
    let warp_get_token = warp::path!("api" / "token" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_token.clone();
            async move { api::token::get(id, cloned).await }
        });

    let db_clone_create_token = lock_db.clone();
    let warp_create_token = warp::path!("api" / "token")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = db_clone_create_token.clone();
            async move { api::token::create(req_json, cloned).await }
        });

    let db_clone_get_resume = lock_db.clone();
    let warp_get_resume = warp::path!("api" / "resume" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_resume.clone();
            async move { api::resume::get(id, cloned).await }
        });

    let db_clone_create_resume = lock_db.clone();
    let warp_create_resume = warp::path!("api" / "resume")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token: Uuid, req_json| {
            let cloned = db_clone_create_resume.clone();
            log::warn!("{}", token);
            async move { api::resume::create(req_json, token, cloned).await }
        });

    let db_clone_get_vacancy = lock_db.clone();
    let warp_get_vacancy = warp::path!("api" / "vacancy" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_vacancy.clone();
            async move { api::vacancy::get(id, cloned).await }
        });

    let db_clone_create_vacancy = lock_db.clone();
    let warp_create_vacancy = warp::path!("api" / "vacancy")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token: Uuid, req_json| {
            let cloned = db_clone_create_vacancy.clone();
            log::warn!("{}", token);
            async move { api::vacancy::create(req_json, token, cloned).await }
        });

    let db_clone_get_response = lock_db.clone();
    let warp_get_response = warp::path!("api" / "response" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_response.clone();
            async move { api::response::get(id, cloned).await }
        });

    let db_clone_create_response = lock_db.clone();
    let warp_create_response = warp::path!("api" / "response")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = db_clone_create_response.clone();
            async move { api::response::create(req_json, cloned).await }
        });

    let db_clone_get_comment = lock_db.clone();
    let warp_get_comment = warp::path!("api" / "comment" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_comment.clone();
            async move { api::comment::get(id, cloned).await }
        });

    let db_clone_create_comment = lock_db.clone();
    let warp_create_comment = warp::path!("api" / "comment")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token: Uuid, req_json| {
            let cloned = db_clone_create_comment.clone();
            async move { api::comment::create(req_json, token, cloned).await }
        });

    let warp_final = warp_hardsql
        //user
        .or(warp_get_user)
        .or(warp_create_user)
        //token
        .or(warp_get_token)
        .or(warp_create_token)
        // resume
        .or(warp_get_resume)
        .or(warp_create_resume)
        //vacancy
        .or(warp_get_vacancy)
        .or(warp_create_vacancy)
        //response
        .or(warp_get_response)
        .or(warp_create_response)
        //comment
        .or(warp_get_comment)
        .or(warp_create_comment);

    return warp_final;
}

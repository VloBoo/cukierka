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

    //////////////////////////////////////

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

    let db_clone_update_user = lock_db.clone();
    let warp_update_user = warp::path!("api" / "user" / Uuid)
        .and(warp::put())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |id, token, req_json| {
            let cloned = db_clone_update_user.clone();
            async move { api::user::update(id, token, req_json, cloned).await }
        });

    let db_clone_delete_user = lock_db.clone();
    let warp_delete_user = warp::path!("api" / "user" / Uuid)
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_user.clone();
            async move { api::user::delete(id, token, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_token = lock_db.clone();
    let warp_create_token = warp::path!("api" / "token")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = db_clone_create_token.clone();
            async move { api::token::create(req_json, cloned).await }
        });

    let db_clone_get_token = lock_db.clone();
    let warp_get_token = warp::path!("api" / "token" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_token.clone();
            async move { api::token::get(id, cloned).await }
        });

    let db_clone_delete_token = lock_db.clone();
    let warp_delete_token = warp::path!("api" / "token")
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |token| {
            let cloned = db_clone_delete_token.clone();
            async move { api::token::delete(token, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_vacancy = lock_db.clone();
    let warp_create_vacancy = warp::path!("api" / "vacancy")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token, req_json| {
            let cloned = db_clone_create_vacancy.clone();
            log::warn!("{}", token);
            async move { api::vacancy::create(req_json, token, cloned).await }
        });

    let db_clone_get_vacancy = lock_db.clone();
    let warp_get_vacancy = warp::path!("api" / "vacancy" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_vacancy.clone();
            async move { api::vacancy::get(id, cloned).await }
        });

    let db_clone_update_vacancy = lock_db.clone();
    let warp_update_vacancy = warp::path!("api" / "vacancy" / Uuid)
        .and(warp::put())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |id, token, req_json| {
            let cloned = db_clone_update_vacancy.clone();
            async move { api::vacancy::update(id, token, req_json, cloned).await }
        });

    let db_clone_delete_vacancy = lock_db.clone();
    let warp_delete_vacancy = warp::path!("api" / "vacancy" / Uuid)
        .and(warp::put())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_vacancy.clone();
            async move { api::vacancy::delete(id, token, cloned).await }
        });

    let db_clone_search_vacancy = lock_db.clone();
    let warp_search_vacancy = warp::path!("api" / "vacancy" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req_json| {
            let cloned = db_clone_search_vacancy.clone();
            async move { api::vacancy::search(req_json, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_response = lock_db.clone();
    let warp_create_response = warp::path!("api" / "response")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token, req_json| {
            let cloned = db_clone_create_response.clone();
            async move { api::response::create(req_json, token, cloned).await }
        });

    let db_clone_get_response = lock_db.clone();
    let warp_get_response = warp::path!("api" / "response" / Uuid)
        .and(warp::get())
        .and_then(move |id| {
            let cloned = db_clone_get_response.clone();
            async move { api::response::get(id, cloned).await }
        });

    let db_clone_delete_response = lock_db.clone();
    let warp_delete_response = warp::path!("api" / "response" / Uuid)
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_response.clone();
            async move { api::response::delete(id, token, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_project = lock_db.clone();
    let warp_create_project = warp::path!("api" / "project")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token, req_json| {
            let cloned = db_clone_create_project.clone();
            async move { api::project::create(req_json, token, cloned).await }
        });

    let db_clone_get_project = lock_db.clone();
    let warp_get_project = warp::path!("api" / "project" / Uuid)
        .and(warp::get())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_get_project.clone();
            async move { api::project::get(id, token, cloned).await }
        });

    let db_clone_delete_project = lock_db.clone();
    let warp_delete_project = warp::path!("api" / "project" / Uuid)
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_project.clone();
            async move { api::project::delete(id, token, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_message = lock_db.clone();
    let warp_create_message = warp::path!("api" / "message")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token, req_json| {
            let cloned = db_clone_create_message.clone();
            async move { api::message::create(req_json, token, cloned).await }
        });

    let db_clone_get_message = lock_db.clone();
    let warp_get_message = warp::path!("api" / "message" / Uuid)
        .and(warp::get())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_get_message.clone();
            async move { api::message::get(id, token, cloned).await }
        });

    let db_clone_delete_message = lock_db.clone();
    let warp_delete_message = warp::path!("api" / "message" / Uuid)
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_message.clone();
            async move { api::message::delete(id, token, cloned).await }
        });

    //////////////////////////////////////

    let db_clone_create_comment = lock_db.clone();
    let warp_create_comment = warp::path!("api" / "comment")
        .and(warp::post())
        .and(warp::header("Token"))
        .and(warp::body::json())
        .and_then(move |token: Uuid, req_json| {
            let cloned = db_clone_create_comment.clone();
            async move { api::comment::create(req_json, token, cloned).await }
        });

    let db_clone_get_comment = lock_db.clone();
    let warp_get_comment = warp::path!("api" / "comment" / Uuid)
        .and(warp::get())
        .and_then(move |id: Uuid| {
            let cloned = db_clone_get_comment.clone();
            async move { api::comment::get(id, cloned).await }
        });

    let db_clone_delete_comment = lock_db.clone();
    let warp_delete_comment = warp::path!("api" / "comment" / Uuid)
        .and(warp::delete())
        .and(warp::header("Token"))
        .and_then(move |id, token| {
            let cloned = db_clone_delete_comment.clone();
            async move { api::comment::delete(id, token, cloned).await }
        });

    //////////////////////////////////////

    let warp_final = warp_hardsql
        //user
        .or(warp_create_user)
        .or(warp_get_user)
        .or(warp_update_user)
        .or(warp_delete_user)
        //token
        .or(warp_create_token)
        .or(warp_get_token)
        .or(warp_delete_token)
        //vacancy
        .or(warp_create_vacancy)
        .or(warp_get_vacancy)
        .or(warp_update_vacancy)
        .or(warp_delete_vacancy)
        .or(warp_search_vacancy)
        //response
        .or(warp_create_response)
        .or(warp_get_response)
        .or(warp_delete_response)
        //project
        .or(warp_create_project)
        .or(warp_get_project)
        .or(warp_delete_project)
        //message
        .or(warp_create_message)
        .or(warp_get_message)
        .or(warp_delete_message)
        //comment
        .or(warp_create_comment)
        .or(warp_get_comment)
        .or(warp_delete_comment);

    return warp_final;
}

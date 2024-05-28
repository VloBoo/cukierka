use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author_id: Uuid,
    pub user_id: Uuid,
    pub rate: i32,
    pub content: String,
    pub created: chrono::DateTime<Utc>,
}

//
// Create Comment
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub user_id: Uuid,
    pub rate: i32,
    pub content: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    comment: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&DeleteResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Comments 
        (id, author_id, user_id, rate, content, create) 
        VALUES ($1, ($2, $3, $4, $5, $6) 
        RETURNING id",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(user_id)
    .bind(req_json.user_id)
    .bind(req_json.rate)
    .bind(req_json.content)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                comment: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: "error".to_string(),
                comment: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Get Comment
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    comment: Option<Comment>,
}
pub async fn get(
    id: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Comments WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                comment: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                comment: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Delete Message
//

#[derive(serde::Serialize)]
pub struct DeleteResponse {
    status: String,
}
pub async fn delete(
    id: Uuid,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&DeleteResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "DELETE FROM Comments WHERE id = $1 and author_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(_) => {
            let res = DeleteResponse {
                status: "ok".to_string(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = DeleteResponse {
                status: error.to_string(),
            };
            Ok(warp::reply::json(&res))
        }
    }
}

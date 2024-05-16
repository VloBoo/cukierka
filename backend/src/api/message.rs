use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author_id: Uuid,
    pub project_id: Uuid,
    pub content: String,
    pub file: Option<Uuid>,
    pub created: chrono::DateTime<Utc>,
}

//
// Create Message
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub project_id: Uuid,
    pub content: String,
    pub file: Uuid,
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
    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Message 
        (id, author_id, project_id, content, file, create) 
        VALUES ($1, (SELECT user_id FROM Tokens WHERE id = $2), $3, $4, $5, $6) 
        RETURNING id",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(token)
    .bind(req_json.project_id)
    .bind(req_json.content)
    .bind(req_json.file)
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
// Get Message
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    comment: Option<Message>,
}
pub async fn get(id: Uuid, db: Arc<Mutex<Database>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Message WHERE id = $1")
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

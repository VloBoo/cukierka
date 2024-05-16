use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

//
// Create Token
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub email: String,
    pub password: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    token: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Tokens (id, user_id, created) 
        VALUES ($1, (SELECT id FROM Users WHERE email = $2 AND password = $3), $4) 
        RETURNING id;",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(req_json.email)
    .bind(req_json.password)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                token: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: "error".to_string(),
                token: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Get Token
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    token: Option<Token>,
}
pub async fn get(
    id: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Token WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                token: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                token: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

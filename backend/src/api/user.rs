use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub information: Option<String>,
    pub created: chrono::DateTime<Utc>,
}

//
// Create User
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub information: Option<String>,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    user: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("INSERT INTO Users (id, name, email, password, information, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(uuid::Uuid::new_v4())
        .bind(req_json.name)
        .bind(req_json.email)
        .bind(req_json.password)
        .bind(req_json.information)
        .bind(Utc::now())
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => Ok(warp::reply::json(&CreateResponse {
            status: "ok".to_string(),
            user: value.try_get("id").ok(),
        })),
        Err(error) => {
            log::error!("{:?}", error);
            Ok(warp::reply::json(&CreateResponse {
                status: "error".to_string(),
                user: None,
            }))
        }
    }
}

//
// Get User
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    user: Option<User>,
}
pub async fn get(
    id: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Users WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                user: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                user: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

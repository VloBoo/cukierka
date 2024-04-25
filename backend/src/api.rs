use crate::{apitool, database::Database};
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

//
// Common struct
//

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub information: Option<String>,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Resume {
    pub id: Uuid,
    pub author_id: Uuid,
    pub portfolio: Option<String>,
    pub skill: Vec<String>,
    pub location: Option<String>,
    pub information: String,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vacancy {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub information: String,
    pub payment: i32,
    pub skill: Vec<String>,
    pub status: String,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub resume_id: Uuid,
    pub vacancy_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub vacancy_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author_id: Uuid,
    pub project_id: Uuid,
    pub content: String,
    pub file: Option<Uuid>,
    pub created: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created: chrono::DateTime<Utc>,
}

//
// Create User
//

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub information: Option<String>,
}
#[derive(serde::Serialize)]
pub struct CreateUserResponse {
    status: String,
    user: Option<Uuid>,
}
pub async fn create_user(
    req_json: CreateUserRequest,
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
        Ok(value) => Ok(warp::reply::json(&CreateUserResponse {
            status: "ok".to_string(),
            user: value.try_get("id").ok(),
        })),
        Err(error) => {
            log::error!("{:?}", error);
            Ok(warp::reply::json(&CreateUserResponse {
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
pub struct GetUserResponse {
    status: String,
    user: Option<User>,
}
pub async fn get_user(
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
            log::warn!("{:?}", apitool::row_to_value(&value).unwrap());
            Ok(warp::reply::json(&GetUserResponse {
                status: "ok".to_string(),
                user: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            }))
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(warp::reply::json(&GetUserResponse {
                status: "error".to_string(),
                user: None,
            }))
        }
    }
}

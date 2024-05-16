use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

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

//
// Create Resume
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub portfolio: Option<String>,
    pub skill: Vec<String>,
    pub location: Option<String>,
    pub information: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    resume: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Resumes 
        (id, author_id, portfolio, skill, location, information, created) 
        VALUES ($1, (SELECT user_id FROM Tokens WHERE id = $2), $3, $4, $5, $6, $7) 
        RETURNING id",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(token)
    .bind(req_json.portfolio)
    .bind(req_json.skill)
    .bind(req_json.location)
    .bind(req_json.information)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                resume: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: "error".to_string(),
                resume: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Get Resume
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    resume: Option<Resume>,
}
pub async fn get(
    id: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Resumes WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                resume: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                resume: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}
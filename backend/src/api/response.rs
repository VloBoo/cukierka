use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub resume_id: Uuid,
    pub vacancy_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

//
// Create Response
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub resume_id: Uuid,
    pub vacancy_id: Uuid,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    response: Option<Uuid>,
}
// TODO: Добавить првоерку статуса
pub async fn create(
    req_json: CreateRequest,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Responses (id, resume_id, vacancy_id, created) 
        VALUES ($1, $2, $3, $4) 
        RETURNING id;",
    )
    .bind(Uuid::new_v4())
    .bind(req_json.resume_id)
    .bind(req_json.vacancy_id)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                response: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: "error".to_string(),
                response: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Get Response
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    response: Option<Response>,
}

pub async fn get(id: Uuid, db: Arc<Mutex<Database>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Responses WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                response: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                response: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

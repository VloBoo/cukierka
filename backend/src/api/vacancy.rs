use super::apitool;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

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

//
// Create Vacancy
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub information: String,
    pub payment: i32,
    pub skill: Vec<String>,
    pub status: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    vacancy: Option<Uuid>,
}
// TODO: Добавить првоерку статуса
pub async fn create(
    req_json: CreateRequest,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Vacancies (id, author_id, title, information, payment, skill, status, created) 
        VALUES ($1, (SELECT user_id FROM Tokens WHERE id = $2), $3, $4, $5, $6, $7, $8) 
        RETURNING id;",
    )
    .bind(Uuid::new_v4())
    .bind(token)
    .bind(req_json.title)
    .bind(req_json.information)
    .bind(req_json.payment)
    .bind(req_json.skill)
    .bind(req_json.status)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                vacancy: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: "error".to_string(),
                vacancy: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Get Vacancy
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    vacancy: Option<Vacancy>,
}

pub async fn get(id: Uuid, db: Arc<Mutex<Database>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Vacancies WHERE id = $1")
        .bind(id)
        .fetch_one(&db_lock.pool)
        .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                vacancy: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: "error".to_string(),
                vacancy: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

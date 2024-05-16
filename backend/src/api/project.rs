use super::apitool;
use crate::api::vacancy;
use crate::database::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub vacancy_id: Uuid,
    pub created: chrono::DateTime<Utc>,
}

//
// Create Project
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub user_id: Uuid,
    pub vacancy_id: Uuid,
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
        "INSERT INTO Projects
            (id, user_id, vacancy_id, created) 
            SELECT $1, $3, $4, $5
            FROM Vacancies v
            INNER JOIN Tokens t ON v.author_id = t.user_id
            WHERE v.id = $4 AND t.id = $2
            RETURNING id;",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(token)
    .bind(req_json.user_id)
    .bind(req_json.vacancy_id)
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
// Get Profect
//

#[derive(serde::Serialize)]
pub struct GetResponse {
    status: String,
    comment: Option<Project>,
}
pub async fn get(id: Uuid, db: Arc<Mutex<Database>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query("SELECT * FROM Projects WHERE id = $1")
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

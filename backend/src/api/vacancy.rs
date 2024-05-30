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
        "INSERT INTO Vacancies (id, author_id, title, information, payment, status, created) 
        VALUES ($1, (SELECT user_id FROM Tokens WHERE id = $2 LIMIT 1), $3, $4, $5, $6, $7) 
        RETURNING id;",
    )
    .bind(Uuid::new_v4())
    .bind(token)
    .bind(req_json.title)
    .bind(req_json.information)
    .bind(req_json.payment)
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
                status: error.to_string(),
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
                status: error.to_string(),
                vacancy: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Update Vacancy
//

#[derive(serde::Deserialize)]
pub struct UpdateRequest {
    pub title: String,
    pub information: String,
    pub payment: i32,
    pub status: String,
}
#[derive(serde::Serialize)]
pub struct UpdateResponse {
    status: String,
    vacancy: Option<Uuid>,
}
// TODO: Добавить првоерку статуса
pub async fn update(
    id: Uuid,
    token: Uuid,
    req_json: UpdateRequest,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&UpdateResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
            vacancy: None,
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "UPDATE Vacancies SET title = $3, information = $4, payment = $5, status = $6 WHERE id = $1 AND author_id = $2 RETURNING id;"
    )
    .bind(id)
    .bind(user_id)
    .bind(req_json.title)
    .bind(req_json.information)
    .bind(req_json.payment)
    .bind(req_json.status)
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = UpdateResponse {
                status: "ok".to_string(),
                vacancy: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = UpdateResponse {
                status: error.to_string(),
                vacancy: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Delete Vacancy
//

#[derive(serde::Deserialize)]
pub struct DeleteRequest {
    pub title: String,
    pub information: String,
    pub payment: i32,
    pub status: String,
}
#[derive(serde::Serialize)]
pub struct DeleteResponse {
    status: String,
}
// TODO: Добавить првоерку статуса
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

    match sqlx::query("DELETE FROM Vacancies WHERE id = $1, author_id = $2;")
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

//
// Search Vacancy
//

#[derive(serde::Deserialize)]
pub struct SearchRequest {
    pub title: String,
    pub sort_by: String,
    pub order: String,
}
#[derive(serde::Serialize)]
pub struct SearchResponse {
    status: String,
    vacancy: Option<Vec<Uuid>>,
}
// TODO: Добавить првоерку статуса
pub async fn search(
    req_json: SearchRequest,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().await;

    match sqlx::query(&format!(
        "SELECT id FROM Vacancies 
    WHERE title ILIKE '%' || $1 || '%' 
    ORDER BY {} {};",
        req_json.sort_by, req_json.order,
    ))
    .bind(req_json.title)
    .bind(req_json.sort_by)
    .bind(req_json.order)
    .fetch_all(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = SearchResponse {
                status: "ok".to_string(),
                vacancy: value.iter().map(|item| item.try_get("id").ok()).collect(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = SearchResponse {
                status: error.to_string(),
                vacancy: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

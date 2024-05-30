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
    pub created: chrono::DateTime<Utc>,
}

//
// Create Message
//

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub project_id: Uuid,
    pub content: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    message: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&CreateResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
            message: None,
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Messages (id, author_id, project_id, content, created)
        SELECT $1, $2, $3, $4, $5
        FROM Projects p
        JOIN Vacancies v ON v.id = p.vacancy_id
        LEFT JOIN Responses r ON r.vacancy_id = p.vacancy_id AND r.user_id = $2
        WHERE p.id = $3 AND (v.author_id = $2 OR r.id IS NOT NULL)
        RETURNING id;",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(user_id)
    .bind(req_json.project_id)
    .bind(req_json.content)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                message: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: error.to_string(),
                message: None,
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
    message: Option<Message>,
}
pub async fn get(
    id: Uuid,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&GetResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
            message: None,
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "SELECT m.*
        FROM Messages m
        JOIN Projects p ON p.id = m.project_id
        JOIN Vacancies v ON v.id = p.vacancy_id
        LEFT JOIN Responses r ON r.vacancy_id = p.vacancy_id AND r.user_id = $2
        WHERE m.id = $1 AND (v.author_id = $2 OR r.id IS NOT NULL);",
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                message: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: error.to_string(),
                message: None,
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
        "DELETE FROM Messages WHERE id = $1 and author_id = $2",
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

use super::apitool;
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
    pub response_id: Uuid,
    pub vacancy_id: Uuid,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    status: String,
    project: Option<Uuid>,
}
pub async fn create(
    req_json: CreateRequest,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&CreateResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
            project: None,
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "INSERT INTO Projects
            (id, user_id, vacancy_id, created) 
            SELECT $1, (SELECT id FROM Responses WHERE id = $4 AND vacancy_id = $3 LIMIT 1), $3, $5
            FROM Vacancies v
            WHERE v.id = $3 AND v.author_id = $2
            RETURNING id;",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(user_id)
    .bind(req_json.vacancy_id)
    .bind(req_json.response_id)
    .bind(Utc::now())
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = CreateResponse {
                status: "ok".to_string(),
                project: value.try_get("id").ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = CreateResponse {
                status: error.to_string(),
                project: None,
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
    project: Option<Project>,
}
pub async fn get(
    id: Uuid,
    token: Uuid,
    db: Arc<Mutex<Database>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(user_id) = apitool::check_token(db.clone(), token).await else {
        return Ok(warp::reply::json(&GetResponse {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
            project: None,
        }));
    };

    let db_lock = db.lock().await;

    match sqlx::query(
        "DELETE FROM Projects
            USING Vacancies v
            WHERE Projects.id = $1
            AND v.id = Projects.vacancy_id
            AND v.author_id = $2;",
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(&db_lock.pool)
    .await
    {
        Ok(value) => {
            let res = GetResponse {
                status: "ok".to_string(),
                project: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
            };
            Ok(warp::reply::json(&res))
        }
        Err(error) => {
            log::error!("{:?}", error);
            let res = GetResponse {
                status: error.to_string(),
                project: None,
            };
            Ok(warp::reply::json(&res))
        }
    }
}

//
// Delete Project
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
        "DELETE Projects
        INNER JOIN Vacancies v ON v.id = p.vacancy_id
        WHERE p.id = $1 AND v.author_id = $2;",
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

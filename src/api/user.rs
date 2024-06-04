use super::apitool;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sqlx::Row;
use sqlx::{Pool, Postgres};
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

#[derive(serde::Deserialize, Debug)]
pub struct CreateRequestParams {
    pub name: String,
    pub email: String,
    pub password: String,
    pub information: Option<String>,
}
#[derive(serde::Serialize)]
pub struct CreateResponseParams {
    status: String,
    user: Option<Uuid>,
}
pub async fn create(
    State(dbx): State<Pool<Postgres>>,
    Json(params): Json<CreateRequestParams>,
) -> impl IntoResponse {
    match sqlx::query("INSERT INTO Users (id, name, email, password, information, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(uuid::Uuid::new_v4())
        .bind(params.name.clone())
        .bind(params.email.clone())
        .bind(params.password.clone())
        .bind(params.information.clone())
        .bind(Utc::now())
        .fetch_one(&dbx)
        .await
    {
        Ok(value) => Json(CreateResponseParams {
            status: "ok".to_string(),
            user: value.try_get("id").ok(),
        }),
        Err(error) => {
            log::error!("{:?}", error);
            Json(CreateResponseParams {
                status: error.to_string(),
                user: None,
            })
        }
    }
}

//
// Get User
//

#[derive(serde::Serialize)]
pub struct GetResponseParams {
    status: String,
    user: Option<User>,
}
pub async fn get(Path(id): Path<Uuid>, State(dbx): State<Pool<Postgres>>) -> impl IntoResponse {
    match sqlx::query("SELECT * FROM Users WHERE id = $1")
        .bind(id)
        .fetch_one(&dbx)
        .await
    {
        Ok(value) => Json(GetResponseParams {
            status: "ok".to_string(),
            user: serde_json::from_value(apitool::row_to_value(&value).unwrap()).ok(),
        }),
        Err(error) => {
            log::error!("{:?}", error);
            Json(GetResponseParams {
                status: error.to_string(),
                user: None,
            })
        }
    }
}

//
// Update User
//

#[derive(serde::Deserialize, Debug)]
pub struct UpdateRequestParams {
    pub name: String,
    pub email: String,
    pub password: String,
    pub information: Option<String>,
}
#[derive(serde::Serialize)]
pub struct UpdateResponseParams {
    status: String,
}

pub async fn update(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(params): Json<UpdateRequestParams>,
    State(dbx): State<Pool<Postgres>>,
) -> impl IntoResponse {
    let Some(user_id) = apitool::check_token(&dbx, headers).await else {
        return Json(UpdateResponseParams {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
        });
    };

    if id != user_id {
        return Json(UpdateResponseParams {
            status: "Доступ запрещен".to_string(),
        });
    }

    match sqlx::query(
        "UPDATE Users SET name = $1, email = $2, password = $3, information = $4 WHERE id = $5",
    )
    .bind(params.name)
    .bind(params.email)
    .bind(params.password)
    .bind(params.information)
    .bind(id)
    .execute(&dbx)
    .await
    {
        Ok(_) => Json(UpdateResponseParams {
            status: "ok".to_string(),
        }),
        Err(error) => {
            log::error!("{:?}", error);
            Json(UpdateResponseParams {
                status: error.to_string(),
            })
        }
    }
}

//
// Delete User
//

#[derive(serde::Serialize)]
pub struct DeleteResponseParams {
    status: String,
}

pub async fn delete(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    State(dbx): State<Pool<Postgres>>,
) -> impl IntoResponse {
    let Some(user_id) = apitool::check_token(&dbx, headers).await else {
        return Json(DeleteResponseParams {
            status: "Не удалось проверить авторизацию пользователя".to_string(),
        });
    };

    if id != user_id {
        return Json(DeleteResponseParams {
            status: "Доступ запрещен".to_string(),
        });
    }

    match sqlx::query("DELETE FROM Users WHERE id = $1")
        .bind(id)
        .execute(&dbx)
        .await
    {
        Ok(_) => Json(DeleteResponseParams {
            status: "ok".to_string(),
        }),
        Err(error) => {
            log::error!("{:?}", error);
            Json(DeleteResponseParams {
                status: error.to_string(),
            })
        }
    }
}

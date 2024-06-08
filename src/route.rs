use axum::{
    extract::Request,
    middleware::{self},
    response::{IntoResponse, Response},
    routing::*,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{api, database::Database};

pub(crate) fn route(dbx: Pool<Postgres>) -> Router {
    let route_v1 = Router::new()
        //user
        .route("/user", post(api::user::create))
        .route("/user/:id", get(api::user::get))
        .route("/user/:id", put(api::user::update))
        .route("/user/:id", delete(api::user::delete));

    Router::new()
        .route("/hardsql", post(hardsql))
        .nest("/api/v1", route_v1)
        .with_state(dbx)
        // .layer(CookieManagerLayer::new())
        .layer(middleware::map_response(logger_res))
        .layer(middleware::map_request(logger_req))
}

async fn logger_res(res: Response) -> Response {
    log::debug!("<<<< {:#?}", res);
    res
}

async fn logger_req(req: Request) -> Request {
    log::debug!(">>>> {:#?}", req);
    req
}
/*
#[derive(Debug)]
struct Id(Option<Uuid>);

#[async_trait]
impl<S> FromRequestParts<S> for Id
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(req, state).await?;
        let a = match req.extensions.get::<Arc<Model>>() {
            Some(_) => "Ok",
            None => "No",
        };
        log::debug!("!!!! {:#?},{:#?},{:#?}", cookies, req, a);
        //let visited = cookies.get(COOKIE_NAME)

        Ok(Id(Some(Uuid::new_v4())))
    }
}
*/
#[derive(Deserialize, Debug)]
struct HardSqlParams {
    sql: String,
}

async fn hardsql(params: Json<HardSqlParams>) -> impl IntoResponse {
    Json(json!({
        "sql": format!("{:?}",Database::hardsql(&params.sql).await)
    }))
}

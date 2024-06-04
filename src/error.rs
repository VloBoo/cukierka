use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Deserialize;

// pub type Result<T> = core::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
pub enum Error{
    Common
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        log::error!("<<<< {:#?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLER_CLIENT_ERROR").into_response()
    }
}
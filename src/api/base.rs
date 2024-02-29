use serde_json::{json, Value};
use uuid::Uuid;

use crate::db;

pub async fn get_user_by_id(uuid: &Uuid) -> Value {
    log::debug!("get_user_by_id");
    let a = match db::get_user(uuid).await {
        Ok(value) => value,
        Err(_) => {
            json!({"Error": "Error"})
        }
    };
    log::trace!("'get_user_by_id' output:{:?}", a);
    return a;
}

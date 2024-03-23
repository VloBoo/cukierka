use serde_json::{self, json, Value};
use user::User;
use uuid::Uuid;

use crate::api::base::Base;
use crate::db::DB;

pub mod user;

pub struct Core {
    users: Vec<User>,
}

impl Core {
    pub fn new() -> Self {
        Core { users: vec![] }
    }

    pub fn get_user<'a>(&'a self, id: Uuid) -> Option<&'a User> {
        //for user in self.users.iter() {
        //    if id.eq(&user.id) {
        //        return Some(user);
        //    }
        //}
        return None;
    }
}

impl Base for Core {
     fn get_user_by_id(self, uuid: &Uuid) -> Value {
        log::debug!("get_user_by_id");

        log::warn!("Небезопасная проверка создания бд");
        let db = DB::new()
            .await
            .inspect_err(|e| log::error!("{:?}", e))
            .expect("Не удалось подключиться к базе данных");

        let a = match db.get_user(uuid).await {
            Ok(value) => value,
            Err(_) => {
                json!({"Error": "Error"})
            }
        };
        log::trace!("'get_user_by_id' output:{:?}", a);
        return a;
    }
}

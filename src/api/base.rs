use serde_json::Value;
use uuid::Uuid;

pub trait Base {
    fn get_user_by_id(self, uuid: &Uuid) -> Value;
}

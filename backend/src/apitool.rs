use chrono::Utc;
use serde_json::json;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::Column;
use sqlx::Row;
use std::error::Error;
use uuid::Uuid;

pub fn row_to_value(row: &PgRow) -> Result<Value, Box<dyn Error>> {
    let mut row_json = serde_json::Map::new();

    for (i, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        let value = if let Ok(value) = row.try_get::<String, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<Uuid, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<chrono::DateTime<Utc>, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<chrono::NaiveDate, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<i32, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<i8, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<i64, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<Vec<String>, usize>(i) {
            json!(value)
        } else if let Ok(value) = row.try_get::<Value, usize>(i) {
            json!(value)
        } else {
            Value::Null
        };
        row_json.insert(column_name.into(), value);
    }
    Ok(serde_json::to_value(row_json)?)
}

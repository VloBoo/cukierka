use chrono::Utc;
use serde_json::{json, Value};
use sqlx::postgres::PgPool;
use sqlx::{Column, Pool, Postgres, Row};
use std::error::Error;
use uuid::Uuid;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Database, Box<dyn Error>> {
        let connection_str = "postgres://akreikuc:ackurkeiiekrukca@vlobo.site:4445/cukierka";
        let pool = PgPool::connect(connection_str).await?;
        log::info!("open connection with database");
        Ok(Database { pool })
    }

    pub async fn hardsql(sql: &str) -> Result<Value, Box<dyn Error>> {
        let db = Database::new().await?;
        let rows = sqlx::query(sql).fetch_all(&db.pool).await?;

        let mut rows_json = Vec::new();

        for row in &rows {
            let mut row_json = serde_json::Map::new();

            for (i, column) in row.columns().iter().enumerate() {
                let column_name = column.name();
                let value = if let Ok(value) = row.try_get::<String, usize>(i) {
                    json!(value)
                } else if let Ok(value) = row.try_get::<Uuid, usize>(i) {
                    json!(value.to_string())
                } else if let Ok(value) = row.try_get::<chrono::DateTime<Utc>, usize>(i) {
                    json!(value.to_rfc3339())
                } else if let Ok(value) = row.try_get::<chrono::NaiveDate, usize>(i) {
                    json!(value.to_string())
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

            rows_json.push(Value::Object(row_json));
        }
        let result = json!({
            "rows": rows_json
        });
        
        Ok(result)
    }
}

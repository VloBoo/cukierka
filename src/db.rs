use serde_json::{json, Value};
use tokio_postgres::NoTls;
use uuid::Uuid;

pub async fn get_tables() -> Result<String, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=VloBo dbname=cukierka", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });

    let rows = client
        .query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';",
            &[],
        )
        .await?;

    let mut result = String::new();
    for row in &rows {
        let table_name: &str = row.get(0);
        result.push_str(table_name);
        result.push_str("\n");
    }

    Ok(result)
}

pub async fn get_user(id: &Uuid) -> Result<Value, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=VloBo dbname=cukierka", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM Users WHERE id = $1::UUID;", &[id])
        .await?;

    let mut rows_json = Vec::new();

    for row in &rows {
        let mut row_json = serde_json::Map::new();

        for i in 0..row.len() {
            let column_name = row.columns()[i].name();
            let value: Value = match row.try_get::<_, String>(i) {
                Ok(value) => json!(value),
                Err(error) => json!(format!("Не удалось получить тип данных {}", error)),
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

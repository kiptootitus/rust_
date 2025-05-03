use tokio_postgres::{NoTls, Error};
use std::env;
use dotenvy::dotenv;

use crate::helpers::helpersmod;

pub async fn get_user_full_names() -> Result<Vec<String>, Error> {
    // Load .env
    dotenv().ok();

    let user = env::var("PG_USER").unwrap();
    let password = env::var("PG_PASSWORD").unwrap();
    let db = env::var("PG_DB").unwrap();
    let host = env::var("PG_HOST").unwrap_or("localhost".to_string());
    let port = env::var("PG_PORT").unwrap_or("5432".to_string());

    let conn_str = format!(
        "host={} user={} password={} dbname={} port={}",
        host, user, password, db, port
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT first, middle, last FROM users", &[])
        .await?;

    let full_names = rows
        .iter()
        .map(|row| {
            let first: &str = row.get(0);
            let middle: &str = row.get(1);
            let last: &str = row.get(2);
            helpersmod::get_full_name(first, middle, last)
        })
        .collect();

    Ok(full_names)
}

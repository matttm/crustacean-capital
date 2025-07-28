pub mod handlers;
pub mod models;
pub mod queries;
pub mod services;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use queries::{CREATE_TABLE_ACCOUNT, CREATE_TABLE_TRANSACTION, CREATE_TABLE_USER};
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let conn = Connection::open_in_memory()?;
    tracing::info!("Established in-memory database connection");
    conn.execute(CREATE_TABLE_USER, ())?;
    conn.execute(CREATE_TABLE_ACCOUNT, ())?;
    conn.execute(CREATE_TABLE_TRANSACTION, ())?;
    tracing::info!("Initializing tables");

    let db = Arc::new(Mutex::new(conn));
    let user_router = Router::new().route("/", get(handlers::user_handlers::get_users));
    // build our application with a route
    let app = Router::new().nest("/users", user_router).with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    println!("Listening on localhost:3000");
    Ok(())
}

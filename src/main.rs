mod queries;
    mod models;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use queries::{CREATE_TABLE_ACCOUNT, CREATE_TABLE_TRANSACTION, CREATE_TABLE_USER};
use rusqlite::{Connection, Result};

async fn get_users() -> Vec<User> {
    vec![]
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let conn = Connection::open_in_memory()?;
    conn.execute(CREATE_TABLE_USER, ());
    conn.execute(CREATE_TABLE_ACCOUNT, ());
    conn.execute(CREATE_TABLE_TRANSACTION, ());

    let user_router = Router::new()
	.route("", method_router)
    // build our application with a route
    let app = Router::new()
	.nest("/users", router)

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

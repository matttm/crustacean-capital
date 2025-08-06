pub mod handlers;
pub mod models;
pub mod queries;
pub mod services;

use axum::{
    Router,
    routing::{get, post},
};
use queries::{CREATE_TABLE_ACCOUNT, CREATE_TABLE_TRANSACTION, CREATE_TABLE_USER};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let pool = SqlitePool::connect("sqlite::inmemory:").await?;
    let conn = pool.acquire().await?;
    tracing::info!("Established in-memory database connection");
    sqlx::query(CREATE_TABLE_USER).execute(&pool).await;
    sqlx::query(CREATE_TABLE_ACCOUNT).execute(&pool).await;
    sqlx::query(CREATE_TABLE_TRANSACTION).execute(&pool).await;
    tracing::info!("Initializing tables");

    let user_router = Router::new()
        .route("/", get(handlers::user_handlers::get_users))
        .route("/", post(handlers::user_handlers::create_user));
    let account_router = Router::new()
        .route("/", get(handlers::account_handlers::get_accounts))
        .route("/", post(handlers::account_handlers::create_account));
    let transaction_router = Router::new()
        .route("/", get(handlers::transaction_handlers::get_transactions))
        .route(
            "/",
            post(handlers::transaction_handlers::create_transaction),
        );
    // build our application with a route
    let app = Router::new()
        .nest("/users", user_router)
        .nest("/accounts", account_router)
        .nest("/transactions", transaction_router)
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    println!("Listening on localhost:3000");
    Ok(())
}

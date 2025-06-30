
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod database;
use database::Db;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db::new()?;
    db.init_table()?;

    let app = Router::new()
        .route("/", get(handler).post(create_user))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}

#[debug_handler]
async fn handler() -> &'static str {
    "Hello, World!"
}

#[debug_handler]
async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = User {
        id: rand::random::<i64>(),
        name: payload.name,
    };

    db.insert_user(user.id, &user.name).unwrap();

    Json(user)
}

#[debug_handler]
async fn list_users(
    State(db): State<Db>,
) -> Json<Vec<User>> {
    let users_data = db.get_all_users().unwrap();
    let users: Vec<User> = users_data.into_iter().map(|(id, name)| User { id, name }).collect();
    Json(users)
}

#[debug_handler]
async fn get_user(
    State(db): State<Db>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    match db.get_user_by_id(id) {
        Ok(Some((id, name))) => Ok(Json(User { id, name })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[debug_handler]
async fn update_user(
    State(db): State<Db>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    match db.update_user(id, &payload.name) {
        Ok(0) => Err(StatusCode::NOT_FOUND),
        Ok(_) => Ok(Json(User { id, name: payload.name })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[debug_handler]
async fn delete_user(
    State(db): State<Db>,
    Path(id): Path<i64>,
) -> StatusCode {
    match db.delete_user(id) {
        Ok(0) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

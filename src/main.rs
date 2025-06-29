
use axum::{
    debug_handler,
    extract::State,
    routing::get,
    Json,
    Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<User>(100);

    tokio::spawn(async move {
        while let Some(user) = rx.recv().await {
            println!("Received user: {:?}", user);
        }
    });

    let app = Router::new()
        .route("/", get(handler).post(create_user))
        .with_state(tx);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn handler() -> &'static str {
    "Hello, World!"
}

#[debug_handler]
async fn create_user(
    State(tx): State<mpsc::Sender<User>>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = User {
        id: rand::random::<u64>(),
        name: payload.name,
    };

    tx.send(user.clone()).await.unwrap();

    Json(user)
}

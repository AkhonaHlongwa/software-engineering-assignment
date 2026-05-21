use axum::Router;
use tokio::net::TcpListener;

#[path = "../api/book_api.rs"]
mod book_api;

#[tokio::main]
async fn main() {

    let app: Router =
        book_api::create_router();

    let listener =
        TcpListener::bind(
            "127.0.0.1:3000"
        )
        .await
        .unwrap();

    println!(
        "Server running on \
        http://127.0.0.1:3000"
    );

    axum::serve(listener, app)
        .await
        .unwrap();
}

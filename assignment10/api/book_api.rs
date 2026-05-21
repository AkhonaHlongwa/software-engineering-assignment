use axum::{
    routing::{get, post},
    Json,
    Router,
};

use std::sync::Mutex;
use once_cell::sync::Lazy;

#[path = "../src/book.rs"]
mod book;

use book::Book;

static BOOKS: Lazy<Mutex<Vec<Book>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub fn create_router() -> Router {

    Router::new()
        .route("/api/books", get(get_books))
        .route("/api/books", post(create_book))
}

async fn get_books() -> Json<Vec<Book>> {

    let books =
        BOOKS.lock().unwrap();

    Json(books.clone())
}

async fn create_book(
    Json(book): Json<Book>,
) -> Json<Book> {

    let mut books =
        BOOKS.lock().unwrap();

    books.push(book.clone());

    Json(book)
}

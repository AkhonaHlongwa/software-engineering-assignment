use axum::{
    routing::{get, post},
    Json,
    Router,
};

use std::sync::Mutex;
use once_cell::sync::Lazy;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../src/book.rs"]
mod book;

use book::Book;

static BOOKS: Lazy<Mutex<Vec<Book>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

#[utoipa::path(
    get,
    path = "/api/books",
    responses(
        (status = 200, description = "Get all books")
    )
)]
async fn get_books() -> Json<Vec<Book>> {

    let books =
        BOOKS.lock().unwrap();

    Json(books.clone())
}

#[utoipa::path(
    post,
    path = "/api/books",
    request_body = Book,
    responses(
        (status = 200, description = "Create book")
    )
)]
async fn create_book(
    Json(book): Json<Book>,
) -> Json<Book> {

    let mut books =
        BOOKS.lock().unwrap();

    books.push(book.clone());

    Json(book)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_books,
        create_book
    ),
    components(
        schemas(Book)
    ),
    tags(
        (name = "Books")
    )
)]
pub struct ApiDoc;

pub fn create_router() -> Router {

    Router::new()
        .route("/api/books", get(get_books))
        .route("/api/books", post(create_book))
        .merge(
            SwaggerUi::new("/docs")
                .url(
                    "/api-doc/openapi.json",
                    ApiDoc::openapi()
                )
        )
}

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};

use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::Serialize;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../src/book.rs"]
mod book;

use book::Book;

// Structured error response template for API clients
#[derive(Serialize)]
struct ApiErrorResponse {
    status: u16,
    error: String,
    message: String,
}

static BOOKS: Lazy<Mutex<Vec<Book>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

#[utoipa::path(
    get,
    path = "/api/books",
    responses(
        (status = 200, description = "Get all books")
    )
)]
async fn get_books() -> impl IntoResponse {
    // Safely attempt to lock the shared in-memory data store
    let books = match BOOKS.lock() {
        Ok(guard) => guard,
        Err(_) => {
            let error_body = ApiErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: "Internal Server Error".to_string(),
                message: "Failed to safely access the book registry data store.".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_body)).into_response();
        }
    };

    (StatusCode::OK, Json(books.clone())).into_response()
}

#[utoipa::path(
    post,
    path = "/api/books",
    request_body = Book,
    responses(
        (status = 201, description = "Book created successfully"),
        (status = 400, description = "Invalid book data provided"),
        (status = 500, description = "Internal data store synchronization error")
    )
)]
async fn create_book(
    Json(book): Json<Book>,
) -> impl IntoResponse {
    
    // 1. Client-Side Input Validation Check
    // Ensures no crucial identifiers or text fields are submitted blank
    if book.book_id.trim().is_empty() {
        let error_body = ApiErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Bad Request".to_string(),
            message: "Validation Error: The 'book_id' field cannot be blank.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error_body)).into_response();
    }

    if book.title.trim().is_empty() {
        let error_body = ApiErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Bad Request".to_string(),
            message: "Validation Error: The book 'title' field cannot be blank.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error_body)).into_response();
    }

    if book.isbn.trim().is_empty() {
        let error_body = ApiErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Bad Request".to_string(),
            message: "Validation Error: The 'isbn' identifier field cannot be blank.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error_body)).into_response();
    }

    if book.status.trim().is_empty() {
        let error_body = ApiErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Bad Request".to_string(),
            message: "Validation Error: The book 'status' field cannot be blank.".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(error_body)).into_response();
    }

    // 2. Safe Mutex Locking & Backend Error Handling
    let mut books = match BOOKS.lock() {
        Ok(guard) => guard,
        Err(_) => {
            let error_body = ApiErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: "Internal Server Error".to_string(),
                message: "The server encountered an error managing shared data locks.".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_body)).into_response();
        }
    };

    books.push(book.clone());

    // 3. Return a clean REST-compliant 201 Created status code on completion
    (StatusCode::CREATED, Json(book)).into_response()
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
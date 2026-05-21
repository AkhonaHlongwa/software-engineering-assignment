#[path = "../src/book.rs"]
mod book;

#[path = "repository.rs"]
mod repository;

use book::Book;
use repository::Repository;

pub trait BookRepository:
    Repository<Book, String> {}

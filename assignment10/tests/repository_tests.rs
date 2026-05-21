#[path = "../repositories/inmemory/inmemory_book_repository.rs"]
mod inmemory_book_repository;

#[path = "../src/book.rs"]
pub mod book;

use inmemory_book_repository::InMemoryBookRepository;
use crate::book::Book;

#[test]
fn test_save_book() {
    let mut repo =
        InMemoryBookRepository::new();

    let book = Book::new(
        String::from("1"),
        String::from("Rust Programming"),
        String::from("ISBN001"),
        String::from("Available"),
    );

    repo.save(book);

    assert_eq!(repo.find_all().len(), 1);
}

#[test]
fn test_find_book_by_id() {
    let mut repo =
        InMemoryBookRepository::new();

    let book = Book::new(
        String::from("1"),
        String::from("Rust Book"),
        String::from("ISBN002"),
        String::from("Available"),
    );

    repo.save(book);

    let found =
        repo.find_by_id(&String::from("1"));

    assert!(found.is_some());
}

#[test]
fn test_delete_book() {
    let mut repo =
        InMemoryBookRepository::new();

    let book = Book::new(
        String::from("1"),
        String::from("Delete Test"),
        String::from("ISBN003"),
        String::from("Available"),
    );

    repo.save(book);

    repo.delete(&String::from("1"));

    assert_eq!(repo.find_all().len(), 0);
}

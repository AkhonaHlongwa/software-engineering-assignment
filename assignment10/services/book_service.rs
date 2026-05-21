#[path = "../repositories/inmemory/inmemory_book_repository.rs"]
mod inmemory_book_repository;

#[path = "../src/book.rs"]
mod book;

use inmemory_book_repository::InMemoryBookRepository;
use book::Book;

pub struct BookService {
    repository: InMemoryBookRepository,
}

impl BookService {

    pub fn new() -> Self {
        Self {
            repository:
                InMemoryBookRepository::new(),
        }
    }

    pub fn create_book(
        &mut self,
        book: Book,
    ) {
        self.repository.save(book);
    }

    pub fn get_all_books(
        &self,
    ) -> Vec<&Book> {
        self.repository.find_all()
    }

    pub fn checkout_book(
        &mut self,
        id: &String,
    ) -> Result<(), String> {

        let book =
            self.repository.find_by_id(id);

        match book {
            Some(found_book) => {

                if found_book.status
                    == "CheckedOut"
                {
                    return Err(
                        String::from(
                            "Book already checked out"
                        )
                    );
                }

                Ok(())
            }

            None => Err(
                String::from(
                    "Book not found"
                )
            ),
        }
    }
}

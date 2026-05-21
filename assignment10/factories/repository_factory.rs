#[path = "../repositories/inmemory/inmemory_book_repository.rs"]
mod inmemory_book_repository;

use inmemory_book_repository::InMemoryBookRepository;

pub struct RepositoryFactory;

impl RepositoryFactory {
    pub fn get_book_repository(
        storage_type: &str,
    ) -> Option<InMemoryBookRepository> {

        match storage_type {
            "MEMORY" => {
                Some(
                    InMemoryBookRepository::new()
                )
            }

            "DATABASE" => {
                println!(
                    "Database repository \
                    not implemented yet"
                );

                None
            }

            _ => None,
        }
    }
}

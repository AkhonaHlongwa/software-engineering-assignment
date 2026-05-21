use std::collections::HashMap;

use crate::book::Book;

pub struct InMemoryBookRepository {
    storage: HashMap<String, Book>,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, book: Book) {
        self.storage
            .insert(book.book_id.clone(), book);
    }

    pub fn find_by_id(
        &self,
        id: &String,
    ) -> Option<&Book> {
        self.storage.get(id)
    }

    pub fn find_all(&self) -> Vec<&Book> {
        self.storage.values().collect()
    }

    pub fn delete(&mut self, id: &String) {
        self.storage.remove(id);
    }
}

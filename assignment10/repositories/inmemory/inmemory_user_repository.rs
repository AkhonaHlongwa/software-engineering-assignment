use std::collections::HashMap;

#[path = "../../src/user.rs"]
mod user;

use user::User;

pub struct InMemoryUserRepository {
    storage: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn save(&mut self, user: User) {
        self.storage
            .insert(user.user_id.clone(), user);
    }

    pub fn find_by_id(
        &self,
        id: &String,
    ) -> Option<&User> {
        self.storage.get(id)
    }

    pub fn find_all(&self) -> Vec<&User> {
        self.storage.values().collect()
    }

    pub fn delete(&mut self, id: &String) {
        self.storage.remove(id);
    }
}

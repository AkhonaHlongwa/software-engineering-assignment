#[path = "../repositories/inmemory/inmemory_user_repository.rs"]
mod inmemory_user_repository;

#[path = "../src/user.rs"]
mod user;

use inmemory_user_repository::InMemoryUserRepository;
use user::User;

pub struct UserService {
    repository: InMemoryUserRepository,
}

impl UserService {

    pub fn new() -> Self {
        Self {
            repository:
                InMemoryUserRepository::new(),
        }
    }

    pub fn create_user(
        &mut self,
        user: User,
    ) {
        self.repository.save(user);
    }

    pub fn get_all_users(
        &self,
    ) -> Vec<&User> {
        self.repository.find_all()
    }
}

#[path = "../src/user.rs"]
mod user;

#[path = "repository.rs"]
mod repository;

use user::User;
use repository::Repository;

pub trait UserRepository:
    Repository<User, String> {}

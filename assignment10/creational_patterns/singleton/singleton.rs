use once_cell::sync::Lazy;

pub struct DatabaseConnection {
    pub connection_string: String,
}

impl DatabaseConnection {
    fn new() -> Self {
        Self {
            connection_string: String::from(
                "LibraryDBConnection"
            ),
        }
    }
}

pub static INSTANCE: Lazy<DatabaseConnection> =
    Lazy::new(|| DatabaseConnection::new());

pub struct Librarian {
    pub librarian_id: String,
    pub username: String,
    pub password: String,
}

impl Librarian {
    pub fn new(
        librarian_id: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            librarian_id,
            username,
            password,
        }
    }

    pub fn approve_loan(&self) {
        println!("Loan approved");
    }

    pub fn generate_report(&self) {
        println!("Generating library report");
    }
}

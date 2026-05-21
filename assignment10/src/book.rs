pub struct Book {
    pub book_id: String,
    pub title: String,
    pub isbn: String,
    pub status: String,
}

impl Book {
    pub fn new(
        book_id: String,
        title: String,
        isbn: String,
        status: String,
    ) -> Self {
        Self {
            book_id,
            title,
            isbn,
            status,
        }
    }

    pub fn check_out(&self) {
        println!("Book checked out");
    }

    pub fn return_book(&self) {
        println!("Book returned");
    }

    pub fn reserve(&self) {
        println!("Book reserved");
    }
}

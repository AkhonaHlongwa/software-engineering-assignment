pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub membership_status: String,
}

impl User {
    pub fn new(
        user_id: String,
        name: String,
        email: String,
        membership_status: String,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            membership_status,
        }
    }

    pub fn borrow_book(&self) {
        println!("{} borrowed a book", self.name);
    }

    pub fn return_book(&self) {
        println!("{} returned a book", self.name);
    }

    pub fn reserve_book(&self) {
        println!("{} reserved a book", self.name);
    }

    pub fn pay_fine(&self) {
        println!("{} paid a fine", self.name);
    }
}

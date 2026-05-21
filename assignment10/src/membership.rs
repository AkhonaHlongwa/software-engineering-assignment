pub struct Membership {
    pub membership_id: String,
    pub expiry_date: String,
    pub status: String,
}

impl Membership {
    pub fn new(
        membership_id: String,
        expiry_date: String,
        status: String,
    ) -> Self {
        Self {
            membership_id,
            expiry_date,
            status,
        }
    }

    pub fn renew_membership(&self) {
        println!("Membership renewed");
    }

    pub fn cancel_membership(&self) {
        println!("Membership cancelled");
    }
}

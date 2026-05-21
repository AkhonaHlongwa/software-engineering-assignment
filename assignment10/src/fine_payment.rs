pub struct FinePayment {
    pub payment_id: String,
    pub amount: f64,
    pub payment_status: String,
}

impl FinePayment {
    pub fn new(
        payment_id: String,
        amount: f64,
        payment_status: String,
    ) -> Self {
        Self {
            payment_id,
            amount,
            payment_status,
        }
    }

    pub fn process_payment(&self) {
        println!("Payment processed");
    }

    pub fn validate_payment(&self) {
        println!("Payment validated");
    }
}

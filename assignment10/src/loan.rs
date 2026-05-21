pub struct Loan {
    pub loan_id: String,
    pub due_date: String,
    pub return_date: String,
}

impl Loan {
    pub fn new(
        loan_id: String,
        due_date: String,
        return_date: String,
    ) -> Self {
        Self {
            loan_id,
            due_date,
            return_date,
        }
    }

    pub fn calculate_fine(&self) {
        println!("Calculating overdue fine");
    }

    pub fn close_loan(&self) {
        println!("Loan closed");
    }
}

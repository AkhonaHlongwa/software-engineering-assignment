pub struct LoanService;

impl LoanService {

    pub fn validate_loan_limit(
        current_loans: usize,
    ) -> Result<(), String> {

        if current_loans >= 5 {
            return Err(
                String::from(
                    "User cannot borrow more than 5 books"
                )
            );
        }

        Ok(())
    }
}

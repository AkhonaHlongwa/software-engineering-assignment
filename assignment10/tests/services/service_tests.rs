#[path = "../../services/loan_service.rs"]
mod loan_service;

use loan_service::LoanService;

#[test]
fn test_valid_loan_limit() {

    let result =
        LoanService::validate_loan_limit(3);

    assert!(result.is_ok());
}

#[test]
fn test_invalid_loan_limit() {

    let result =
        LoanService::validate_loan_limit(5);

    assert!(result.is_err());
}

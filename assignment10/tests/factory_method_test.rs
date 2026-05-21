#[path = "../creational_patterns/factory_method/factory_method.rs"]
mod factory_method;

use factory_method::{
    CreditCardFactory,
    PayPalFactory,
    PaymentFactory,
};

#[test]
fn test_credit_card_processor_creation() {
    let factory = CreditCardFactory;
    let _processor = factory.create_processor();
}

#[test]
fn test_paypal_processor_creation() {
    let factory = PayPalFactory;
    let _processor = factory.create_processor();
}

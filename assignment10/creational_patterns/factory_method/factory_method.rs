pub trait PaymentProcessor {
    fn process_payment(&self);
}

pub struct CreditCardProcessor;

impl PaymentProcessor for CreditCardProcessor {
    fn process_payment(&self) {
        println!("Processing credit card payment");
    }
}

pub struct PayPalProcessor;

impl PaymentProcessor for PayPalProcessor {
    fn process_payment(&self) {
        println!("Processing PayPal payment");
    }
}

pub trait PaymentFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor>;
}

pub struct CreditCardFactory;

impl PaymentFactory for CreditCardFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(CreditCardProcessor)
    }
}

pub struct PayPalFactory;

impl PaymentFactory for PayPalFactory {
    fn create_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(PayPalProcessor)
    }
}

#[path = "../creational_patterns/builder/builder.rs"]
mod builder;

use builder::PizzaBuilder;

#[test]
fn test_build_basic_pizza() {
    let pizza = PizzaBuilder::new()
        .add_cheese()
        .build();

    assert!(pizza.cheese);
    assert!(!pizza.pepperoni);
}

#[test]
fn test_build_full_pizza() {
    let pizza = PizzaBuilder::new()
        .add_cheese()
        .add_pepperoni()
        .add_mushrooms()
        .build();

    assert!(pizza.cheese);
    assert!(pizza.pepperoni);
    assert!(pizza.mushrooms);
}

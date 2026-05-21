#[path = "../creational_patterns/simple_factory/simple_factory.rs"]
mod simple_factory;

use simple_factory::VehicleFactory;

#[test]
fn test_create_car() {
    let _car = VehicleFactory::create_vehicle("car");
}

#[test]
fn test_create_bike() {
    let _bike = VehicleFactory::create_vehicle("bike");
}

#[test]
fn test_create_truck() {
    let _truck = VehicleFactory::create_vehicle("truck");
}

pub trait Vehicle {
    fn drive(&self);
}

pub struct Car;

impl Vehicle for Car {
    fn drive(&self) {
        println!("Driving a Car");
    }
}

pub struct Bike;

impl Vehicle for Bike {
    fn drive(&self) {
        println!("Driving a Bike");
    }
}

pub struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Driving a Truck");
    }
}

pub struct VehicleFactory;

impl VehicleFactory {
    pub fn create_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
        match vehicle_type {
            "car" => Box::new(Car),
            "bike" => Box::new(Bike),
            "truck" => Box::new(Truck),
            _ => panic!("Invalid vehicle type"),
        }
    }
}

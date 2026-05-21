#[derive(Debug)]
pub struct Pizza {
    pub cheese: bool,
    pub pepperoni: bool,
    pub mushrooms: bool,
}

pub struct PizzaBuilder {
    cheese: bool,
    pepperoni: bool,
    mushrooms: bool,
}

impl PizzaBuilder {
    pub fn new() -> Self {
        Self {
            cheese: false,
            pepperoni: false,
            mushrooms: false,
        }
    }

    pub fn add_cheese(mut self) -> Self {
        self.cheese = true;
        self
    }

    pub fn add_pepperoni(mut self) -> Self {
        self.pepperoni = true;
        self
    }

    pub fn add_mushrooms(mut self) -> Self {
        self.mushrooms = true;
        self
    }

    pub fn build(self) -> Pizza {
        Pizza {
            cheese: self.cheese,
            pepperoni: self.pepperoni,
            mushrooms: self.mushrooms,
        }
    }
}

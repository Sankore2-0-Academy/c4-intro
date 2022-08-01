/**
 * Traits - Gives Object Interfaces
 */
pub fn run() {
 let car = Car {name: String::from("Mazda")};

 car.reverse();
 car.move_forward();
}

/**
 * Declare a trait
 */
trait Vehicle {
 fn move_forward(&self);
 fn reverse(&self);
}

struct Car {
 name: String,
}

impl Vehicle for Car {
    fn move_forward(&self) {
        println!("{} is moving forward", self.name);
    }

    fn reverse(&self) {
        println!("Reverse {}", self.name);
    }
}
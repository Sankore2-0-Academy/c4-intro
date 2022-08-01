/**
 * Structs - used to create custom data types
 */
pub fn run() {
 let car = Car::new(String::from("Mazda"), String::from("demio"), 2000);

 println!("{:?}", car.name);

 car.move_forward();
}

#[derive(Debug)]
struct Car {
 name: String,
 model: String,
 mileage: u64
}

impl Car {
 pub fn new(name: String, model: String, mileage: u64) -> Self {
  Car {
   name,
   model,
   mileage,
  }
 }

 pub fn move_forward(&self) {
  println!("{} is moving forward", self.name);
 }
}
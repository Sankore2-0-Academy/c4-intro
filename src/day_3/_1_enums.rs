/**
 * Enums are types which have a few definite values
 */
pub fn run() {
 let move_car = CarMove::TURN_LEFT;

 match move_car {
  CarMove::FORWARD => println!("Car is moving forward"),
  CarMove::REVERSE => println!("Car is reversing"),
  CarMove::TURN_LEFT => println!("Car is turning left"),
  CarMove::TURN_RIGHT => println!("Car is turning right"),
  _ => println!("Invalid input")
 }
}

enum CarMove {
 FORWARD,
 REVERSE,
 TURN_LEFT,
 TURN_RIGHT
}


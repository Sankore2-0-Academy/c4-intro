/**
 * What is a variable?
 * 
 * Variables hold primitive data or reference to data
 * Variables are immutable by default
 * Rust is a block-scoped language
 */

pub fn run() {
 // Define literal variables
 let x = 34;
 let word = "Hello";


 // Define Constant variables
 const PIE: f64 = 3.142;

 // Define multiple variables
 let (name, age) = ("Paul", 34);

 println!("{name} is {age} years old");


  // What is a comment?
  // Comments elaborate more about a block or a line of code
  // 1. Documentation comment
  // 2. Inline comment



  // Displaying values in console
  // using print
  // 1. print to console
  println!("Hello World");

  // 2. Basic formatting
  println!("Value: {}", 50);

  // 3. Positional Arguments
  println!("{0} comes from {1} and likes {2} colour", "Brian", "Mombasa", "Blue");

  // 4. Named Arguments
  println!("{name} comes from {city} and likes {colour} colour", colour="Green", name="Destiny", city="Nigeria");

  // 5. Placeholder Traits
  println!(" Binary: {:b}\n Hex: {:x}\n Octal: {:o}", 10, 10, 10);

  // 6. Placeholder for debugging traits
  let arr = vec![1,2,3,45];
  println!("{:?}", arr);

}
/**
 * What is data type?
 * 
 * Primitive Types:--
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 * 
 * NOTE: 
 * Rust is a statically typed language, which means that it must know the types of all variables at 
 * compiletime, however, the compiler can usually infer what type we want to use based on the value and 
 * how we use it.
 */

 pub fn run() {
  // Integers
  // Default is i32
  let x = 34;
  let y: u32 = 34;

  // Float
  // Default is f64
  let price = 100.00;
  let cash: f32 = 100.00;

  // Add type explicitly
  let cash: u128 = 1000000000000000000000000000000;

  // Boolean
  let is_true: bool = true;

  // Boolean from expression
  let bool_val: bool = 45 > 56;

  // Character
  let character: char = 'c';
  let emoji: char = '\u{1F600}';

  // Tuples
  let (name, age): (&str, u32) = ("Flemming", 45);
  println!("{name}: {age}");
 }
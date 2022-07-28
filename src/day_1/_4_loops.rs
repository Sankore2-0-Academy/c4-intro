/**
 * What are control structures?
 * A way of to specify flow of control in a program based on the analysis of variables.
 * 
 * a Loop is a programming iterative logic that executes a block of code until a condition is met.
 */

 pub fn run() {
  // Infinite loop
  let mut counter: i32 = 0;
  loop {

   if counter > 10 {
    break;
   }

   println!("Counter: {}", counter);

   counter += 1;
  }

  // While loop
  let mut count: i32 = 0;
  while count <=10 {
   println!("Count: {}", count);
   count += 1;
  }

  // for loop
  for number in 0..10 {
   println!("Number: {number}");
  }

 }
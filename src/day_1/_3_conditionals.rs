/**
 * What are control structures?
 * A way to specify flow of control in a program based on the analysis of variables.
 * 
 * IF statements is a programming conditional logic that executes a block of code if the condition is met.
 */

 pub fn run() {
  // IF statement
  if 56 > 45 {
   println!("True");
  }

  // IF/ELSE statement
  if 45 > 56 {
   println!("Actually True");
  } else {
   println!("Actually False");
  }

  // IF/ELSE/IF statements
  if 45 > 56 {
   println!("45 > 56");
  } else if 10 > 7 {
   println!("10 > 7");
  } else {
   println!("Else");
  }

  // Shorthand IF statement
  let age = 15;
  let is_candidate: bool = if age >= 18 {true} else {false};
  println!("{is_candidate}");

  // Match statement
  match age {
   18 => {
    println!("Almost there");
   },
   20 => println!("Of age"),
   _ => println!("Invalid age")
  }
 }
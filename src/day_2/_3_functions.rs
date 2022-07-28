/**
 * What is a function?
 */
pub fn run() {
 // Invoke a function
 hey();

 // Passing function arguments
 let student_name = "Destiny";
 greeting(student_name);

 // Save function value to variables
 let res = sum(56, 4673);
 println!("Sum: {res}");

 // Closures
 let callback = || {
  println!("From Closure: {student_name}");
 };

 callback();

 callback_test(|| {
  println!("Hello from closure");
 });

}

/**
 * Declare a function
 */
fn hey() {
 println!("Doing something!!!");
}

fn greeting(name: &str) {
 println!("Hello {name}!");
}

fn sum(num1: i32, num2: i32) -> i32 {
 let result = num1 + num2;
 result
}

fn callback_test(cal: fn() -> ()) {
 println!("From callback test function:");
 cal();
}
/**
 * Array - Fixed list where elements are the same data types
 */
pub fn run() {
 // Define Array
 let mut numbers: [u32;7] = [2,3,4,5,6,7,89];

 // Get value
 let number: u32 = numbers[1];

 // Re-assign value
 numbers[1] = 47;

 // Get length
 let length = numbers.len();

 // Get slice
 let sub_numbers = &numbers[0..4];

 // tuples();

 vectors();

}

/**
 * Tuples group together values of different types
 * Max 12 elements
 */
fn tuples() {
 // Tuple definition
 let mut profile: (&str, u32, &str) = ("Flemming", 67, "Malindi");

 // Get value
 let name = profile.0;

 // Re-assign value
 profile.0 = "Peter";

 profile = ("Peter", 56, "Kilifi");
 println!("{:?}", profile);

}

/**
 * Vectors - Resizable arrays
 */
fn vectors() {
 // Vector definition
 let mut numbers: Vec<u32> = vec![2,3,5,7,89];

 // Get Value
 let number: u32 = numbers[4];

 // Re-assign value
 numbers[2] = 400;

 // Add element
 numbers.push(567);
 numbers.push(300);
 // println!("{:?}", numbers);

 // Remove last element
 numbers.pop();
 // println!("{:?}", numbers);

 numbers.remove(0);
 // println!("{:?}", numbers);

 // Get length
 let size = numbers.len();

 // Get slice
 let sub_vec = &numbers[3..];
 // println!("Sub Vector: {:?}", sub_vec);

 // Loop through elements
 for item in numbers {
  println!("Item: {item}");
 }
 
}


/**
 * Primitive str = Immutable fixed-length string somewhere in memory
 * String = Growable heap-allocated data structure - Use when you need to modify or own string data
 */

pub fn run() {
 // Fixed-length string
 let mut name: &str = "Gard";
 name = "Erick";

 // Growable string
 let mut sentence = String::from("Hello");

 // Length of String
 let len = sentence.len();

 // Push character to string
 sentence.push(' ');

 // Push string
 sentence.push_str("World");

 // Check if empty
 let is_empty = sentence.is_empty();

 // Contains method
 let contains_world = sentence.contains("World");

 //Replace method
 let sentence_v2 = sentence.replace("World", "Peter");
 println!("{sentence_v2}");
 
}
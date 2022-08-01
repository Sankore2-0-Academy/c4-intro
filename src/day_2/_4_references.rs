/**
 * Reference Pointer - Point to aresource in memory.
 */
pub fn run() {
 // Primitive array
 let array = [1,2,3,4,5];
 let array_2 = array;


 //NOTE: With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource.

 // Vectors
 let students = vec!["Samson", "Paul", "Lawrence"];
 let candidates = &students;

 let name = String::from("Gard");
 let mut name_2 = name;

 greet(&mut name_2);

 println!("{:?}", name_2);

}

fn greet(name: &mut String) {
 name.push_str(" Alson");
 println!("Hello {name}")
}
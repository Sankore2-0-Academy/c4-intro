// mod day_1;
// mod day_2;
mod day_3;

fn main() {
    // day_1::main();
    // day_2::main();
    // day_3::main();
    let result = divide(25, 5);
    println!("Division: {result}");
}

pub fn divide(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum() {
        // assert!(exp, string)
        // assert_eq(param1, param2)
        // assert_ne(param1, param2)

        assert!( 1 + 1 == 2 );
        assert_eq!(5,5);
        assert_ne!(5,7);
    }

    #[test]
    fn check_division() {
        let res = divide(25, 5);
        // assert!(res == 5, "Division failed");
        assert_eq!(res, 5);
    }
}
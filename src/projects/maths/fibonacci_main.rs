/**
 * FIBONACCI Calculator
 * 
 */
pub mod fibonacci {
    use std::io;

    pub fn main() {

        let mut user_val = String::new();

        println!("Enter a number to get the Fibonacci value for:");
        io::stdin().read_line(&mut user_val).expect("Unable to read user value");

        let user_val: u32 = match user_val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                0 as u32
            }
        };

        println!("You entered {} the Fibonacci values is: {}"
        ,user_val
        ,get_next_fibonacci(user_val) );

    }

    fn get_next_fibonacci(num: u32) -> u32 {
        if num <= 1 {
            return 1;
        }
        return get_next_fibonacci(num - 1) + get_next_fibonacci(num - 2);
    }
}
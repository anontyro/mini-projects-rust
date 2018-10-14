/**
 * FIBONACCI Calculator
 * required the user to enter a positive int value to be parse and the
 * fibonacci value will be given
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

    /**
     * Recursive method that takes in a int32 and will return an int64 value
     * for the fibonacci value providing a positive number > 0 is supplied
     */
    fn get_next_fibonacci(num: u32) -> u64 {

        match num {
            0 => panic!("zero is an incorrect arguement"),
            1 | 2 => 1,
            3 => 2,
            _ =>  get_next_fibonacci(num - 1) + get_next_fibonacci(num - 2)
        }
    }
}
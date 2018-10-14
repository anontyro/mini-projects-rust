extern crate main_lib;
use std::io;

fn main() {
    user_select();
}

// User selection logic loop that creates the selection menu for the user
fn user_select() {
    loop {
        println!("
        Select a project from the list:
        -------------------------
        1 = Temperature converter
        2 = Fibonacci calculator
        -------------------------
        or e(x)it" );

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Unable to read user command");

        println!("user input: {}", user_input );

        if user_input.trim() == "x" {
            break;
        } else if user_input.trim() == "1" {
            main_lib::temp_main::temp_convert::main();
        } else if user_input.trim() == "2" {
            main_lib::fibonacci_main::fibonacci::main();
        }


        
    }
}

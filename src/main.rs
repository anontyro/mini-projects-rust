extern crate main_lib;
use std::io;

fn main() {
    // main_lib::test();
    user_select();
}

fn user_select() {
    loop {
        println!("Select a project from the list, or press x to exit" );

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input);

        println!("user input: {}", user_input );

        if user_input.trim() == "x" {
            break;
        } else if user_input.trim() == "1" {
            main_lib::temp_main::temp_convert::temp();
        }


        
    }
}

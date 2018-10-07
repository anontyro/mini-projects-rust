/**
 * TEMPERATURE CONVERTER
 * basic program that will allow the user to convert numbers between different temperatures
 */
pub mod temp_convert {
    use std::io;

    /* Temperature converter entry point to be called and used */
    pub fn main() {

        let mut temp = String::new();
        let mut temp_type = String::new();
        let mut user_agree = String::new();

        println!("Enter the temperature you would like to convert" );
        io::stdin().read_line(&mut temp).expect("Unable to read line");
        let temp = temp.trim();

        println!("You have entered: {} conver to (c)elcius or (f)ahrenheit?", temp );
        io::stdin().read_line(&mut temp_type).expect("Unable to read user answer");
        let temp_type = temp_type.trim();

        println!("You would like to convert {} to {}, (y)es (n)o",temp, temp_type );
        io::stdin().read_line(&mut user_agree).expect("Unable to read user output");
        let user_agree = user_agree.trim();

        let temp: f32 = match temp.parse() {
            Ok(num) => num,
            Err(_) => {
                let _temp_type = "n";
                0 as f32
            },
        };

        if user_agree.to_lowercase() == "y" || 
            user_agree.to_lowercase() == "yes"  {
            temp_convert(temp, &temp_type);
        } else {
            println!("user quit" );
        }

    }

    // contains the control statement that will allow for a selection to convert the temp correctly
    fn temp_convert(temp: f32, temp_type: &str) {
        if temp_type == "c" {
            println!("Temperature in Celcius: {}", fah_to_cel(temp) );
        } else if temp_type == "f" {
            println!("Temperature in Fahrenheit: {}", cel_to_fah(temp) );
        } else {
            println!("User selection not supported");
        }
    }

    // Convert celcius to fahrenheit
    fn cel_to_fah(temp: f32) -> f32 {
        let output: f32 = ((temp * 9./5.) +32.) as f32;
        output
    }

    // Convert fahrenheit to celcius
    fn fah_to_cel(temp: f32) -> f32 {
        let output: f32 = ((temp -32.) * 5./9.) as f32;
        output
    }

}
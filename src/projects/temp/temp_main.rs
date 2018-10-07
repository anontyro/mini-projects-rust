
pub mod temp_convert {
    use std::io;

    pub fn main() {

        let mut temp = String::new();
        let mut temp_type = String::new();
        let mut user_agree = String::new();

        println!("Enter the temperature you would like to convert" );
        io::stdin().read_line(&mut temp);

        println!("You have entered: {} is this (c)elcius or (f)ahrenheit?", temp );
        io::stdin().read_line(&mut temp_type);

        println!("You would like to convert {} to {}, (y)es (n)o",temp, temp_type );
        io::stdin().read_line(&mut user_agree);

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                let temp_type = "n";
                0 as f32
            },
        };

        if user_agree.trim() == "y" || 
            user_agree.trim() == "Y" ||
            user_agree.trim() == "yes"  {
            temp_convert(temp, &temp_type);
        } else {
            println!("user quit" );
        }

    }
    fn temp_convert(temp: f32, temp_type: &str) {
        if temp_type.trim() == "c" {
            println!("Temperature in Celcius: {}", fah_to_cel(temp) );
        } else if temp_type.trim() == "f" {
            println!("Temperature in Fahrenheit: {}", cel_to_fah(temp) );
        } else {
            println!("User selection not supported");
        }
    }

    fn cel_to_fah(temp: f32) -> f32 {
        let output: f32 = ((temp * 9./5.) +32.) as f32;
        output
    }

    fn fah_to_cel(temp: f32) -> f32 {
        let output: f32 = ((temp -32.) *5./9.) as f32;
        output
    }

}
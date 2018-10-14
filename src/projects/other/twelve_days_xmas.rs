
pub mod twelve_days {
    use std::io;

    pub fn main() {

        let mut user_val = String::new();

        println!("Test value: ");
        io::stdin().read_line(&mut user_val).expect("Unable to read line");

        match user_val.trim().parse() {
            Ok(num) =>  twelve_day_out(num),
            Err(_) => panic!("Not a number"),
        };

    }

    fn twelve_day_out(num: u32) {

        match num {
            0 => panic!("number out of range"),
            1 ... 12 => println!(
                "On the {} day of Christmas my true love sent to me {}",
                num,
                get_day_value(num)
            ),
            _ => panic!("out of range")
        };

    }

    fn get_day_value(num: u32) -> String {
        let value_list =
        [
            "A partridge in a pear tree",
            "Two turtle doves",
            "Three French hens",
            "Four calling birds",
            "Five gold rings",
            "Six geese a-laying",
            "Seven swans a-swimming",
            "Eight maids a-milking",
            "Nine ladies dancing",
            "Ten lords a-leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming"
        ];

        value_list[(num - 1) as usize].to_string()
    }


    
}
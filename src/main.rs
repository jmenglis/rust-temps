use std::io;
use std::convert::AsRef;

fn main() {
    let mut input = String::new();
    let mut temp_type = String::new();
    println!("Please enter a temperature: ");
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let converted_temp = input.trim().parse().expect("Please type a temp");
            println!("Please enter the scale: ");
            match io::stdin().read_line(&mut temp_type) {
                Ok(_n) => {
                    let temp_type: bool = match temp_type.as_ref() {
                        "F"=> true,
                        "C"=> false,
                        _ => false,
                    };
                    let final_temp: f32 = handle_temperature_conversion(converted_temp, temp_type);
                    println!("Temperature is converted to {}C", final_temp);
                }
                Err(error) => println!("error: {}", error),
            }
        }
        Err(error) => println!("error: {}", error),
    }
}

fn handle_temperature_conversion(input: f32, temp_type: bool) -> f32 {
    if temp_type {
        (input * 9.0/5.0) + 32.0
    } else {
        println!("HIT");
        (input - 32.0) * 5.0/9.0
    }
}
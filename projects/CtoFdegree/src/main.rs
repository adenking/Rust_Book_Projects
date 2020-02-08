use std::io;

fn main() {
    println!("What is the temperature you would like to convert?");

    let mut input_temp = String::new();

    io::stdin().read_line(&mut input_temp)
        .expect("Failed to read line.");

    let input_temp: u32 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Would you like to convert to (C) or (F)?");

    let mut temp_type = String::new();

    io::stdin().read_line(&mut temp_type)
        .expect("Failed to read line.");

    let mut temp_type = temp_type.trim();

    match temp_type {
        "C" => {
            let c_input_temp = (((input_temp-32)*5)/9);
            println!("{} Celcius", c_input_temp);
        },
        "F" => {
            let f_input_temp = input_temp*9/5+32;
            println!("{} Farenheit", f_input_temp);
        }
        _ => println!("Not valid input"),
    }
}

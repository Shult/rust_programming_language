use std::io;

fn main() {
    println!("This program can convert celsius to fahrenheit, or fahrenheit to celsius");
    
    loop {
        println!("Please enter (C) to convert your degrees to celsius or (F) to convert to fahrenheit...");
        
        let mut convert_to = String::new();
        io::stdin()
            .read_line(&mut convert_to)
            .expect("Failed to read line");

        let convert_to: String = convert_to.trim().parse().expect("REASON");

        if convert_to == "C" {
            let temperature = get_degrees_number();
            let temperature = fahrenheit_to_celsius(temperature);
            println!("{} °C", temperature)

        } else if convert_to == "F" {
            let temperature = get_degrees_number();
            let temperature = celsius_to_fahrenheit(temperature);
            println!("{} °F", temperature)

        } else {
            println!("{}", convert_to);
            println!("Please enter (C) to convert your degrees to celsius or (F) to convert to fahrenheit...");

            continue
        }
    }
}

fn get_degrees_number() -> i32 {
    println!("Please enter the temperature you want to convert...");

    let mut degrees_number = String::new();
    io::stdin()
        .read_line(&mut degrees_number)
        .expect("Failed to read line");

    let degrees_number: i32 = degrees_number.trim().parse().expect("REASON");
    degrees_number
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    let fahrenheit = (celsius*9/5)+32;
    fahrenheit
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let celsius = (fahrenheit-32)*5/9;
    celsius
}
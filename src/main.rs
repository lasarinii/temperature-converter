use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Temperature Converter\n ");
   
    'running: loop {
        println!("Fahrenheit to Celsius [F]\nCelsius to Fahrenheit [C]\n:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Can't read the line.");
        let temp = temp.trim(); 

        if temp == "F" || temp == "f" {
            let input = input_temp();
            let result = fahrenheit_to_celsius(input);
            for _i in 1..4 {
                    println!(".");
                sleep(Duration::from_millis(500));
            }
            println!("{input}°F = {result}°C");
        } else if temp == "C" || temp == "c" {
            let input = input_temp();
            let result = celsius_to_fahrenheit(input);
            for _i in 1..4 {
                println!(".");
                sleep(Duration::from_millis(500));
            }
            println!("{input}°C = {result}°F");
        } else {
            println!("Invalid temperature.");
            continue;
        }

        loop {
            sleep(Duration::from_secs(1));
            println!("Convert again? [Y/N]");
            let mut again = String::new();
            io::stdin()
                .read_line(&mut again)
                .expect("Can't read line.");
            let again = again.trim();

            if again == "Y" || again == "y" {
                break;
            } else if again == "N" || again == "n" {
                break 'running;
            } else {
                continue;
            }
        }
    }
}

fn input_temp() -> f32 {
    loop {
        println!("Temperature: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Can't read the line.");
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input;
    }
}

fn fahrenheit_to_celsius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}

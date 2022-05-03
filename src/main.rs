use std::io;

enum ConverterMode {
    FahrenheitToCelsius,
    CelsiusToFahrenheit,
}

fn ask_conversion_mode() -> ConverterMode {
    loop {
        let mut selection = String::new();

        println!("Please choose the conversion mode.");
        println!("(1) Fahrenheit to Celsius");
        println!("(2) Celsius to Fahrenheit");
        println!("");
        println!("Your selection: ");
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {}
            Err(_) => {
                println!("Bruv something went wrong.");
                continue;
            }
        }

        selection = selection.trim().to_string();

        if selection.eq("1") {
            return ConverterMode::FahrenheitToCelsius;
        } else if selection.eq("2") {
            return ConverterMode::CelsiusToFahrenheit;
        }
        println!("No match for option '{}'", selection)
    }
}

fn ask_input(mode: &ConverterMode) -> f32 {
    loop {
        let mut input = String::new();

        match mode {
            ConverterMode::CelsiusToFahrenheit => {
                println!("Converting Celsius to Fahrenheit.");
                println!("Please enter the temperature in Celsius:");
            }
            ConverterMode::FahrenheitToCelsius => {
                println!("Converting Fahrenheit to Celsius.");
                println!("Please enter the temperature in Fahrenheit:");
            }
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Bruv something went wrong.");
                continue;
            }
        }

        let input = match input.trim().parse::<f32>() {
            Ok(res) => res,
            Err(_err) => {
                println!("Please insert a legit number, thanks!");
                continue;
            }
        };

        return input;
    }
}

fn convert_fahrenheit_to_celsius(value: f32) -> f32 {
    (value - 32.0) / 1.8
}

fn convert_celsius_to_fahrenheit(value: f32) -> f32 {
    (value * 1.8) + 32.0
}

fn main() {
    println!("YOOO DAWG, WANNA CONVERT SOME TEMPERATURES?");
    let mode = ask_conversion_mode();
    let input = ask_input(&mode);

    match mode {
        ConverterMode::CelsiusToFahrenheit => {
            let result = convert_celsius_to_fahrenheit(input);
            println!("{} °C ???? Yo DAWG that shit is {} °F", input, result)
        }
        ConverterMode::FahrenheitToCelsius => {
            let result = convert_fahrenheit_to_celsius(input);
            println!("{} °F ???? Yo DAWG that shit is {} °C", input, result)
        }
    }
}

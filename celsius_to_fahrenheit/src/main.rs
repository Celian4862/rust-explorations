fn main() {
    println!("Which way are you converting?\n1. Celsius to Fahrenheit\n2. Fahrenheit to Celsius");
    let choice: bool = loop {
        let mut choice = String::new();
        match std::io::stdin().read_line(&mut choice) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to read line: {e}");
                continue;
            },
        }
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        match choice {
            1 => break true,
            2 => break false,
            _ => {
            println!("Please enter 1 or 2.");
            continue;
            }
        };
    };

    println!("Enter the temperature: ");
    let temperature: i32 = loop {
        let mut temperature = String::new();
        match std::io::stdin().read_line(&mut temperature) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to read line: {e}");
            },
        }
        match temperature.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
    };

    if choice {
        println!("{temperature} degrees Celsius is {:.2} degrees Fahrenheit.", (temperature as f64 * (9.0/5.0)) + 32.0);
    } else {
        println!("{temperature} degrees Fahrenheit is {:.2} degrees Celsius.", (temperature - 32) as f64 * (5.0/9.0));
    }
}

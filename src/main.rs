use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let number1: f64 = match args[1].parse::<f64>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Enter a valid number.");
            std::process::exit(1);
        }
    };

    println!("First number: {}", number1);

    let operator = match args[2].chars().next() {
        Some('+') => '+',
        Some('-') => '-',
        Some('*') => '*',
        Some('/') => '/',
        Some('%') => '%',
        _ => {
            eprintln!("Invalid operator.");
            std::process::exit(1);
        }
    };

    println!("Operator: {}", operator);

    let number2: f64 = match args[3].parse::<f64>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Enter a valid number.");
            std::process::exit(1);
        }
    };

    println!("Second number: {}", number2);

    match calculate(number1, operator, number2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("{}", error),
    }
}
fn calculate(number1: f64, operator: char, number2: f64) -> Result<f64, String> {
    match operator {
        '+' => Ok(number1 + number2),
        '-' => Ok(number1 - number2),
        '*' => Ok(number1 * number2),
        '/' => {
            if number2 == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(number1 / number2)
            }
        }
        '%' => Ok(number1 % number2),
         _ => Err("Invalid operator".to_string()),
    }
}


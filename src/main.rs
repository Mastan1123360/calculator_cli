use std::io;

fn main() {
    loop {
        let mut input1 = String::new();
        println!("Enter your fist number (or) Enter 'q' to exit: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read input.");
        if input1.trim().to_lowercase() == "q" {
            break;
        }
        let number1: f64 = match input1.trim().parse::<f64>() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        let mut operator = String::new();
        println!("Enter an operator[+ or - or * or / or %]: ");
        io::stdin()
            .read_line(&mut operator)
            .expect("Reading operator input failed.");
        let operator = match operator.trim() {
            "+" => "+",
            "-" => "-",
            "*" => "*",
            "/" => "/",
            "%" => "%",
            _ => {
                println!("Invalid operator.");
                continue;
            }
        };

        let mut input2 = String::new();
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input.");
        let number2: f64 = match input2.trim().parse::<f64>() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Your first number: {}", number1);
        println!("Your oprator: {:?}", operator);
        println!("Your second number: {}", number2);

        match calculate(number1, operator, number2) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("{}", error),
        }
    }
}
fn calculate(number1: f64, operator: &str, number2: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(number1 + number2),
        "-" => Ok(number1 - number2),
        "*" => Ok(number1 * number2),
        "/" => {
            if number2 == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(number1 / number2)
            }
        }
        "%" => Ok(number1 % number2),
         _ => Err("Invalid operator".to_string()),
    }
}

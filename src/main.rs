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
        let number1: f64 = match input1
            .trim()
            .parse::<f64>() {
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
        let operator = operator.trim();

        let mut input2 = String::new();
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input.");
        let number2: f64 = match input2
            .trim()
            .parse::<f64>() {
                Ok(number) => number,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

        println!("Your first number: {}", number1);
        println!("Your oprator: {:?}", operator);
        println!("Your second number: {}", number2);

        match operator {
            "+" => println!("Result: {}", number1 + number2),
            "-" => println!("Result: {}", number1 - number2),
            "*" => println!("Result: {}", number1 * number2),
            "/" => {
                if number2 == 0.0 {
                    println!("Cannot divide with 0");
                } else {
                    println!("Result: {}", number1 / number2);
                }
            }
            "%" => println!("Result: {}", number1 % number2),
            _ => println!("Invalid operator"),
        }
    }
}

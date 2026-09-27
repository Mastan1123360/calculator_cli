use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let number1: f64 = args[1].parse().unwrap();
    println!("First number: {}", number1);
    
    let operator = args[2].chars().next().unwrap();
    println!("Operator: {}", operator);
    
    let number2: f64 = args[3].parse().unwrap();
    println!("Second number: {}", number2);
    
    match calculate(number1, operator, number2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("{}", error),
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


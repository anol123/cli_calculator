use std::{io::{self, Read}};

enum CalError{
    InvalidNumber,
    DivisionByZero,
    InvalidOperator,
}
fn main() {
    loop{
        let input =read_input("Enter your choice: compute or exit");
        if input =="exit".to_string(){
            println!("Goodbye!!!");
            return;
        }
        else{
            let input1 = read_input("Enter first number");

            let num1 = match parse_number(input1){
                Ok(input) =>input,
                Err(_) =>{
                    println!("Invalid number");
                    continue;
                } 
            };

            let num2 = match parse_number( read_input("Enter second number")){
                Ok(input) =>input,
                Err(_) =>{
                    println!("Invalid number");
                    continue;
                } 
            };

            let op =read_input("Enter operator (+, -, *, /)");
        
        match compute(num1, num2, &op){
            Ok(result) =>  println!("Result:  {result}"),
            Err(CalError::DivisionByZero) => println!("Cannot divide by zero"),
            Err(CalError::InvalidOperator) => println!("Please enter a valid operator"),
            Err(CalError::InvalidNumber) => println!("Invalid input entered")
        };
    }
        
     
    }
}

fn compute(num1:f64, num2:f64, op:&str)-> Result<f64, CalError>{
    let result = match op{
        "+" => Ok(num1+num2),
        "-" => Ok(num1-num2),
        "*" => Ok(num1*num2),
        "/" => if(num2==0.0){
            Err(CalError::DivisionByZero)
        }
        else{
            Ok(num1/num2)
        }
        _ => Err(CalError::InvalidOperator),
    };
    return result;
}

fn read_input(prompt:&str)-> String{
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn parse_number(input:String) -> Result<f64, CalError>{
    input.trim().parse::<f64>().map_err(|_| CalError::InvalidNumber)
}
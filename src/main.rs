use std::{io::{self, Read}};

enum CalError{
    InvalidNumber,
    DivisionByZero,
    InvalidOperator,
}
fn main() {
    let num1 =2.0;
    let num2 =3.0;
    let op = "+";

    match compute(num1, num2, op){
        Ok(result) =>  println!("Result:  {result}"),
        Err(CalError::DivisionByZero) => println!("Cannot divide by zero"),
        Err(CalError::InvalidOperator) => println!("Please enter a valid operator"),
        Err(CalError::InvalidNumber) => println!("Invalid input entered")
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
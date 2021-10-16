use std::io::{stdin, stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}


fn main() {
    println!("Welcome to Akin Calculator!");
    println!("---------------------------");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        print!("What is the first number?: ");
        read(&mut num1);

        print!("What is the second number?: ");
        read(&mut num2);
        //println!("{} {} {}", num1, operator, num2);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();

        //loop {
        print!("What operation would you like to do? [+-*/]: ");
        read(&mut operator);
        let operator: char = operator.trim().chars().next().unwrap();

        // let operators = String::from("+-*/");
        // if operators.contains(operator){
        //     break;
        // } else {
        //     println!("Unknown operator");
        // }
        //}
        
        
        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => {
                println!("Unknown operator");
                continue;
            }
            //_ => panic!("error in operator")
        };
        println!("The result of {} {} {} = {}", num1, operator, num2, result);
    }
}

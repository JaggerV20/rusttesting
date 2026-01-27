use std::io;

//Main holds a main loop that asks for a number to select an operation
//User is then prompted to enter two numbers for the operation. Expects integers
fn main() {

    //Not sure if there's a better enumeration option
    const ADD_INT: i32 = 1;
    const SUB_INT: i32 = 2;
    const MULT_INT: i32 = 3;
    const DIV_INT: i32 = 4;
    println!("Starting calculator");
    println!("Enter an operation: ");
    println!("1) Addition");
    println!("2) Subtraction");
    println!("3) Multiplication");
    println!("4) Division");
    let mut operation_choice = String::new();
    io::stdin().read_line(&mut operation_choice).expect("Failed to read line");
    let operation_choice: i32 = operation_choice.trim().parse().expect("Please type a number");
    if operation_choice < 1 || operation_choice > 4 {
        println!("Please enter a number from 1 to 4");
        return;
    }

    println!("Enter the first operand: ");
    let mut operand1 = String::new();
    io::stdin().read_line(&mut operand1).expect("Failed to read line");
    //Shadowing
    let operand1: i32 = operand1.trim().parse().expect("Please type a number");

    println!("Enter the second operand: ");
    let mut operand2 = String::new();
    io::stdin().read_line(&mut operand2).expect("Failed to read line");
    let operand2: i32 = operand2.trim().parse().expect("Please type a number");

    match operation_choice {
        ADD_INT => rust_add(operand1, operand2),
        SUB_INT => rust_sub(operand1, operand2),
        MULT_INT => rust_mult(operand1, operand2),
        DIV_INT => rust_div(operand1, operand2),
        _ => println!("Invalid operation choice. This shouldn't appear"),
    }
}

//This design is pretty unneccesary, but it's an opportunity to practice functions
//Adds x and y. Prints result equation
fn rust_add(x : i32, y : i32){
    //I'm not sure if this is proper, but it works
    println!("{x} + {y} = {}", x + y);
}

//Subtracts y from x. Prints result equation
fn rust_sub(x : i32, y : i32){
    println!("{x} - {y} = {}", x - y);
}

//Multiply x and y. Prints result equation
fn rust_mult(x : i32, y : i32){
    println!("{x} * {y} = {}", x * y)
}

//Divide x by y. Prints result equation
fn rust_div(x : i32, y : i32){
    println!("{x} / {y} = {}", x / y)
}

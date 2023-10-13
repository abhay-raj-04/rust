use std::io;
fn main() {
    println!("\n\n\t\t\tSimple Calculator\n\n");
    loop {
        let mut part1: String = String::new();
        println!("\n→Simple Calculation (Addition,Subtraction,Multiplication,Division,Modulus)\n     press(1).\n→Complex Calculation (Square ,Square Root) \n     press(2)\n→Back \n     press (3)");
        io::stdin().read_line(&mut part1).expect("msg");
        let part1: i32 = part1.trim().parse().expect("msf");
        match part1 {
            1 => simp(),
            2 => comp(),
            3 => {
                println!("To Continue with calculator press(1).\n To exit press (2)");
                let mut not = String::new();
                io::stdin().read_line(&mut not).expect("Error");
                let not: i32 = not.trim().parse().expect("Error");
                if not == 2 {
                    println!("Exiting Calculator......");
                    break;
                }
            }
            _ => println!("error"),
        }
    }
}
fn simp() {
    let mut option: String = String::new();
    println!("For Addition → press (1) \nFor Subtraction → press(2)\nFor Multiplication → press(3)\nFor Division → press(4)\nFor Modulus → press(5)");
    io::stdin().read_line(&mut option).expect("msg");
    let option: i32 = option.trim().parse().expect("msf");
    match option {
        1 => add(),
        2 => sub(),
        3 => mul(),
        4 => div(),
        5 => rem(),
        _ => println!("error"),
    }
}
fn comp() {
    let mut option1: String = String::new();
    println!("For Square press → (1)\nFor Square Root → press (2)");
    io::stdin().read_line(&mut option1).expect("msg");
    let option1: i32 = option1.trim().parse().expect("msf");
    match option1 {
        1 => square(),
        2 => square_root(),
        _ => println!("error"),
    }
}

fn add() {   
    let add = simp_calc();
    let (num1, num2) = add;
    println!("Addition:{}", num1 + num2);
}
fn sub() {  
    let add = simp_calc();
    let (num1, num2) = add;
    println!("Subtraction:{}", num1 - num2);
}
fn mul() {   
    let add = simp_calc();
    let (num1, num2) = add;
    println!("Multiplication:{}", num1 * num2);
}
fn div() {   
    let add = simp_calc();
    let (num1, num2) = add;
    println!("Division:{}", num1 / num2);
}
fn rem() {
    let add = simp_calc();
    let (num1, num2) = add;
    println!("REmainder:{}", num1 % num2);
}
fn square() {
    let sqr = comp_calc();
    println!("Square : {}", sqr * sqr);
}
fn square_root() {
    let square_root = comp_calc();
    println!("Square Root: {}", square_root.sqrt());
}
fn comp_calc() -> f32 {
    let mut _num1 = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut _num1).expect("Try again");
    let _num1: f32 = _num1.trim().parse().expect("Try again.");
    _num1
}
fn simp_calc() -> (f32, f32) {
    let mut num1 = String::new();
    let mut num2 = String::new(); 
    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Try again");
    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Try again");
    let num1: f32 = num1.trim().parse().expect("Try again.");
    let num2: f32 = num2.trim().parse().expect("Try again.");
    (num1, num2)
}

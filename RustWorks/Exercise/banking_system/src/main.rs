const PIN: i64 = 1234;
use serde_json::{json, Value};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
fn main() {
    println!("\n\n\t\t\t=> ATM <=");

    //3 attempts for PIN Insertion
    let mut count: u32 = 0;
    let mut attempt: u32 = 3;
    while count < 3 {
        let mut pin = String::new();
        println!("Enter PIN:");
        io::stdin().read_line(&mut pin).expect("Enter numbers.");
        let pin: i64 = match pin.trim().parse() {
            Ok(num1) => num1,
            Err(_) => continue,
        };
        if pin == PIN {
            break;
        } else {
            attempt -= 1;
            println!("Attempts Remaining:{}", attempt);
            count += 1;
        }
    }
    loop {
        if count == 3 {
            println!("Access Denied.");
            break;
        }

        //Selection
        println!("\n\nSelect:\n ⇶ (1)Check Balance\n ⇶ (2)Withdraw Money\n ⇶ (3)Deposit Money \n ⇶ (4)Exit");
        let (mut notes_500, mut notes_100, mut balance) = notes();
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Enter 1,2,3,4");
        let option: u8 = match option.trim().parse() {
            Ok(num1) => num1,
            Err(_) => continue,
        };
        match option {
            1 => check_balance(),
            2 => withdraw_amount(&mut balance, &mut notes_500, &mut notes_100),
            3 => deposit_amount(&mut balance, &mut notes_500, &mut notes_100),
            4 => {
                exit();
                break;
            }
            _ => println!("Enter 1,2,3,4"),
        }
    }
}
//exit Function
fn exit() {
    println!("\n Cancel  Exit ⇒ (1).\n Confirm Exit ⇒ (2)");
    let mut not = String::new();
    io::stdin().read_line(&mut not).expect("Press 1 or 2");
    let not: u32 = match not.trim().parse() {
        Ok(num1) => num1,
        Err(_) => 4,
    };
    if not == 2 {
        println!("\n\t\t..x..Exiting ATM..x..");
    } else {
        main();
    }
}
fn transaction() {
    println!("\n\tTransaction Successful.");
    check_balance();
}
fn check_balance() {
    //checks Available Balance
    let notes = notes();
    println!("\n\tAvailable Balance:{}", notes.2);
}
fn withdraw_amount(balance: &mut f32, notes_500: &mut u32, notes_100: &mut u32) {
    //withdraw money
    println!("\nEnter the amount you want to withdraw:");
    let withdraw: f32 = input_amount();
    loop {
        if withdraw > *balance {
            println!("Insufficient Balance.");
            withdraw_amount(balance, notes_500, notes_100);
        } else {
            break;
        }
    }
    let num_500: u32 = withdraw as u32 / 500;
    let remaining_after_500 = withdraw as u32 % 500;
    let num_100: u32 = remaining_after_500 / 100;

    if num_500 > *notes_500 {
        let num_500_from_notes = *notes_500;
        let remaining_amount_to_withdraw =
            (num_500 - num_500_from_notes) * 500 + remaining_after_500;
        let num_100_from_notes = remaining_amount_to_withdraw / 100;

        if num_100_from_notes > *notes_100 {
            println!("Insufficient 100-rupee notes.");
            return;
        }
        println!("Withdrawn Rs.500 notes: {}", num_500_from_notes);
        println!("Withdrawn Rs.100 notes: {}", num_100_from_notes);
        *balance -= withdraw;
        *notes_500 -= num_500_from_notes;
        *notes_100 -= num_100_from_notes;
    } else {
        println!("Withdrawn Rs.500 notes: {}", num_500);
        println!("Withdrawn Rs.100 notes: {}", num_100);
        *balance -= withdraw;
        *notes_500 -= num_500;
        *notes_100 -= num_100;
    }

    update_notes(*notes_500, *notes_100, *balance);
    transaction();
}
fn deposit_amount(balance: &mut f32, notes_500: &mut u32, notes_100: &mut u32) {
    println!("\nEnter the amount you want to deposit:");
    let deposit: f32 = input_amount();
    loop {
        println!("Enter number of Rs.500 notes to be deposited:");
        let num_500: u32 = input();
        println!("Enter number of Rs.100 notes to be deposited:");
        let num_100: u32 = input();
        let total_amount: u32 = (num_100 * 100) + (num_500 * 500);
        if total_amount == deposit as u32 {
            *balance += total_amount as f32;
            *notes_500 += num_500;
            *notes_100 += num_100;
            update_notes(*notes_500, *notes_100, *balance);
            break;
        } else {
            println!("Enter number of notes Correctly.");
        }
    }

    transaction();
}
fn input() -> u32 {
    //Amount for Deposit or Withdrawal
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Enter Amount in numbers");
    let input: u32 = input.trim().parse().expect("Error");
    input
}
fn input_amount() -> f32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("enter number");
    let x: f32 = input.trim().parse().expect("failed");
    x
}
fn notes() -> (u32, u32, f32) {
    let mut file = File::open("acc.json").expect("failed to read.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read");

    let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON.");
    let notes_500 = json_data["five_hundred_note"].as_u64().unwrap_or(0) as u32;
    let notes_100 = json_data["one_hundred_note"].as_u64().unwrap_or(0) as u32;
    let balance = json_data["balance"].as_f64().unwrap_or(0.0) as f32;
    (notes_500, notes_100, balance)
}
fn update_notes(notes_500: u32, notes_100: u32, balance: f32) {
    let json_data = json!({
        "balance": balance,
        "five_hundred_note": notes_500,
        "one_hundred_note": notes_100,
    });
    let json_str = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON.");
    let mut file = File::create("acc.json").expect("Failed to create file.");
    file.write_all(json_str.as_bytes())
        .expect("Failed to write to file.");
}

use std::io;
use rand::Rng;
fn main() {
    // println!("Enter first number");
    // let mut user_input = String::new();

    // io::stdin().read_line(&mut user_input).expect("Error reading input");
    // let prase_input:u128 = user_input.trim().parse().expect("Error parsing input");

    // let generated_number:u32 = rand::thread_rng().gen_range(1, 101);
    // println!("{:?}", prase_input);

    // if prase_input == generated_number {
    //     println!("Equality")
    // }
    // else if prase_input > generated_number{
    //     println!("Greater than")
    // }
    // else {
    //     println!("None")
    // }

    //feze buzz game 
    println!("Welcome to Feze Buzz Game");
    println!("Enter your start number");
    let mut start_number: String = String::new();
    io::stdin().read_line(&mut start_number).expect("Error reading");
    let parse_start_number:u32 = start_number.trim().parse().expect("Error parsing");

    println!("Enter your end number");
    let mut end_number = String::new();
    io::stdin().read_line(&mut end_number).expect("Error reading");
    let parse_end_number:u32 = end_number.trim().parse().expect("Error parsing");

    println!("Enter your feze number");
    let mut feze_number = String::new();
    io::stdin().read_line(&mut feze_number).expect("Error reading");
    let parse_feze_number:u32 = feze_number.trim().parse().expect("Error parsing");

    println!("Enter your buzz number");
    let mut buzz_number = String::new();
    io::stdin().read_line(&mut buzz_number).expect("Error reading");
    let parse_buzz_number:u32 = buzz_number.trim().parse().expect("Error parsing");

    for i in parse_start_number..=parse_end_number {
        if i % parse_feze_number == 0  && i % parse_buzz_number == 0{
            println!("FezeBuzz");
        }
        else if i % parse_feze_number == 0 {
            println!("Feze")
        }
        else if i % parse_buzz_number == 0{
            println!("Buzz")
        }
        else {
            println!("{}", i);
        }
    }
    
}

use colored::*;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    println!("Guessing Game!!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!(
            "{}",
            "-----------------Please enter a input!-----------------".bright_blue()
        );
        let mut guess: String = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "!!!!-------------You Winn-------------!!!!".green());
                break;
            }
            Ordering::Less => println!("{}", "Too less".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
        }
    }
}

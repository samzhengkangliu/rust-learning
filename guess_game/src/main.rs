use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude 预导入 // trait 接口

fn main() {
    // '!' means macro
    println!("Welcome to Guess Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please guess a number:");

        // variables are immutable by default in Rust
        // mut is required to reassign variables
        let mut guess = String::new();

        // & means reference
        // reference is also immutable in Rust
        io::stdin().read_line(&mut guess).expect("Cannot read line");
        // io::Result Ok, Err
        // Expect will check if io::result returns Err, print the line that passed in
        // otherwise, print the message in Ok

        // shadow the old 'guess' and error handling case
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The number you guess is: {}", guess);

        // match will run the arm if the params matches any arms in scope
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

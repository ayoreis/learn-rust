use ferris_says::say;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // let stdout = io::stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = io::BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();

    println!("🎲 Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Number please!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🥳 You win!");
                break;
            },
        }
    }  
}

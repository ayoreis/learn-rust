use std::io;
use json;
use std::io::Write;

fn main() {
    let stdin = io::stdin();

    println!("💅 Make some JSON pretty!\n");

    loop {
        let mut json = String::new();

        println!("Type some JSON:\n");
    
        stdin
            .read_line(&mut json)
            .expect("Invalid input");

        let parsed = match json::parse(&json) {
            Ok(value) => {
                value
            },

            Err(_) => {
                println!("\nError! Try again\n");

                continue
            },
        };

        let pretty = json::stringify_pretty(parsed, 4);

        println!("\n{pretty}\n");
        print!("Exit? (y/N) ");
        io::stdout().flush().unwrap();

        let mut exit = String::new();

        stdin
            .read_line(&mut exit)
            .unwrap();

        match exit.to_lowercase().trim() {
            "y" => break,
            _ => {},
        }
    }

    println!("\n👋 Bye! I hope you liked your pretty JSON");
}

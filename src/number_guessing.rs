
use std::io::stdin;

use rand::Rng;

pub(crate) fn start_game() {
    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Try to guess a number between 1 and 100: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range")
                        } else if guess > number {
                            println!("Your guess is too high")
                        } else if guess < number {
                            println!("Your guess is too low")
                        } else {
                            println!("Correct!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e)
                    }
                }
            }
            Err(_) => continue
        }
    }
}

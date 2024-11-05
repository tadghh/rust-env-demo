use rand::seq::SliceRandom;
use std::env;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;

fn main() {
    let words = ["apple", "banana", "cherry", "date", "elderberry"];
    let stdin = stdin();
    let mut stdout = stdout();

    println!("Press enter to update the environment variable or the CRTL+C to exit.");

    for c in stdin.keys() {
        match c {
            Ok(_) => {
                // Choose a random word from the array
                if let Some(&word) = words.choose(&mut rand::thread_rng()) {
                    // A dynamic env that updates when the user presses enter
                    env::set_var("RANDOM_WORD", word);
                    println!("Updated env variable: {}", env::var("RANDOM_WORD").unwrap());

                    // This env is set from the config file in .cargo we cannot access this without using a 'cargo command env'
                    let config_test = env::var("CONFIG_TEST").unwrap_or(
                        "CONFIG_TEST is not set. We aren't running as a Cargo command.".to_owned(),
                    );
                    println!("CONFIG_TEST: {}", config_test);

                    // This env is set with build.rs, it cannot be changed and is embedded at compile time
                    let compile_time_env = option_env!("COMPILE_TIME_ENV").unwrap_or("None");
                    println!("Compile env variable: {}", compile_time_env);
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
            }
        }

        stdout.flush().unwrap(); // Ensure the output is displayed
    }
}

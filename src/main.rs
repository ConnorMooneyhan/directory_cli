use std::{env, fs, process};

fn main() {
    // Collects arguments
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    let rest_args = &args[2..];

    // Writes directory contents to variable
    let contents = match fs::read_to_string("directory.txt") {
        Ok(text) => text,
        Err(_) => String::new()
    };

    // Selects command function to run based on user input
    match command {
        "add" => contact::add(rest_args, &contents),
        "search" => contact::search(rest_args, &contents),
        _ => eprintln!("Invalid command")
    };
}
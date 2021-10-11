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
        "add" => match contact::add(rest_args, &contents) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Unable to add contact :(");
                process::exit(1);
            }
        },
        "search" => {
            let lb = "--------------------";
            println!("{}", lb);
            for result in contact::search(rest_args, &contents) {
                println!(
                    "{}\n{}", 
                    result, 
                    lb
                );
            }
        },
        _ => eprintln!("Invalid command")
    };
}
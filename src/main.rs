use std::{ env, fs, process, collections::HashMap };
use serde_json;

fn main() {
    // Collects arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        contact::print_docs();
        process::exit(1);
    }

    let command = args[1].as_str();
    let rest_args = &args[2..];
    let mut directory_path = env::current_exe().expect("Couldn't find directory.json path.");
    directory_path.pop();
    directory_path.push("directory.json");

    // Writes directory contents to variable
    let mut directory_is_empty = false;
    let contents = match fs::read_to_string(&directory_path) {
        Ok(text) => text,
        Err(_) => {
            directory_is_empty = true;
            String::new()
        }
    };

    // Encodes contents as HashMap
    let mut contacts: HashMap<String, contact::Contact> = match directory_is_empty {
        false => serde_json::from_str(&contents).expect("JSON file not correctly formatted."),
        true => HashMap::new()
    };

    // Selects command function to run based on 'command'
    match command {
        "add" | "a" => contact::add(rest_args, &mut contacts, &directory_path),
        "search" | "view" | "s" => contact::display_contacts(contact::search(rest_args, &contacts)),
        //"delete" => contact::delete(contact::search(rest_args, &contacts)),
        "help" => contact::print_docs(),
        _ => {
            println!("'{}' is not a valid command.", command);
            contact::print_docs();
        }
    };
}
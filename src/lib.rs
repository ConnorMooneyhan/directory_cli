use std::{cmp, collections::HashMap, fmt, fs, path, process};
use serde::{ Serialize, Deserialize };
use serde_json;
use colored::*;

// Every command that accesses a contact should use this struct
// Defines Contact struct
#[derive(Serialize, Deserialize)]
pub struct Contact {
    first: String,
    last: String,
    number: String,
    display_length: usize,
}

impl Contact {
    // Creates new Contact from data
    pub fn new(mut first: String, mut last: String, number: String) -> Contact {
        // Formats input
        first = capitalize(&first.trim().to_string());
        last = capitalize(&last.trim().to_string());

        // Returns new Contact
        Contact {
            first: first.clone(), 
            last: last.clone(),
            number: number.clone(),
            display_length: cmp::max(
                first.len() + last.len() + "Name:  ".len(),
                number.len() + "Number: () -".len()
            )
        }
    }

    // Displays contact dynamically, based on other contact lengths
    pub fn display(&self, length: &usize) -> String {
        format!(
            "Name: {}{} {}\nNumber: {}({}) {}-{}",
            spaces(length - ("Name:  ".len() + self.first.len() + self.last.len())),
            &self.first,
            &self.last,
            spaces(length - ("Number: () -".len() + 10)),
            &self.number[..3],
            &self.number[3..6],
            &self.number[6..]
        )
    }
}

// Implements Display for use in viewing contact
impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Name: {}{} {}\nNumber: {}({}) {}-{}",
            spaces(self.display_length - ("Name:  ".len() + self.first.len() + self.last.len())),
            &self.first,
            &self.last,
            spaces(self.display_length - ("Number: () -".len() + 10)),
            &self.number[..3],
            &self.number[3..6],
            &self.number[6..]
        )
    }
}

// Adds new contact to directory
pub fn add(args: &[String], contacts: &mut HashMap<String, Contact>, path: &path::PathBuf, command: &str) {
    exit_if_empty(args);

    
    // Creates new contact
    let new_contact =  match args.len() {
        3 => Contact::new(
            args[0].clone(),
            args[1].clone(),
            args[2].clone()
        ),
        2 => Contact::new(
            args[0].clone(),
            "UNKNOWN".to_string(),
            args[1].clone()
        ),
        _ => Contact::new(
            String::new(), 
            String::new(), 
            String::new()
        )
    };

    // Stores associated values to be used after new_contact is moved
    let new_length = new_contact.display_length;
    let new_first = new_contact.first.clone();
    let new_last = new_contact.last.clone();
    let new_number = new_contact.number.clone();

    let full_name = format!("{} {}", new_first, new_last);

    if command == "e" || command == "edit" {
        match contacts.get(&full_name) {
            None => {
                println!("{}", format!("No contact with the name \"{}\" was found.", full_name).bright_red());
                return;
            }
            _ => ()
        }
    }

    // Adds contact to contacts
    contacts.insert(
        full_name,
        new_contact
    );

    // Writes updated contacts to directory.json
    let write_result = fs::write(
        path,
        serde_json::to_string_pretty(&contacts).unwrap()
    );
    
    match write_result {
        Ok(_) => (),
        Err(_) => {
            eprintln!("{}", "Unable to add contact :(".bright_red());
            process::exit(1);
        }
    }

    let (message, extra_spacing) = match command {
        "add" | "a" => ("Added", 0),
        "edit" | "e" => ("Updated", 1),
        _ => ("", 0)
    };

    let general_buffer = new_length / 2 - match new_length % 2 {1 => 2, 0 => 3, _ => 0};
    let spacing = spaces(general_buffer);
    let msg_spacing = spaces(general_buffer - extra_spacing);

    println!(
        "\n{}{}\n  {}|\n  {}|\n  {}V",
        msg_spacing,
        message.bright_green(),
        spacing,
        spacing,
        spacing
    );
    display_contacts(&vec![Contact::new(
        new_first,
        new_last,
        new_number
    )]);
}    

// Searches directory for contact information to print
pub fn search(args: &[String], contacts: &HashMap<String, Contact>) -> Vec<Contact> {
    exit_if_empty(args);
    let mut matches = Vec::new();
    
    // Processes search term
    let search_term = match args.len() {
        1 => args[0].clone(),
        2 => format!("{} {}", args[0], args[1]),
        _ => {
            eprintln!("{}", "Please enter a name or part of a name to search.".bright_red());
            process::exit(1);
        }    
    };    
    
    // Searches entries
    for key in contacts.keys() {
        if key.to_lowercase().contains(&search_term.to_lowercase()) {
            let value = contacts.get(key).unwrap();
            matches.push(Contact::new(
                value.first.clone(),
                value.last.clone(),
                value.number.clone()
            ));    
        }    
    }    
    
    // Returns vector of matches
    matches
}

// Searches for matches by number
pub fn reverse_search(args: &[String], contacts: &HashMap<String, Contact>) -> Vec<Contact> {
    exit_if_empty(args);
    let mut matches = Vec::new();
    
    // Processes search term
    let search_term = match args.len() {
        1 => args[0].clone(),
        _ => {
            eprintln!("{}", "Please enter a number or part of a number to reverse search.".bright_red());
            process::exit(1);
        }    
    };    
    
    // Searches entries
    for contact in contacts.values() {
        if contact.number.contains(&search_term.to_lowercase()) {
            matches.push(Contact::new(
                contact.first.clone(),
                contact.last.clone(),
                contact.number.clone()
            ));    
        }    
    }    
    
    // Returns vector of matches
    matches
}

// Deletes contact from directory
pub fn delete(args: &[String], contacts: &mut HashMap<String, Contact>, path: &path::PathBuf) {
    exit_if_empty(args);
    let matches = search(args, &contacts);
    // Runs cases for number of matches
    match matches.len() {
        1 => {
            contacts.remove(&format!("{} {}", matches[0].first, matches[0].last));
            let write_result = fs::write(
                path,
                serde_json::to_string_pretty(&contacts).unwrap()
            );    
            
            match write_result {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("{}", "Unable to delete contact :(".bright_red());
                    process::exit(1);
                }    
            }    

            let display_length = vec_max_length(&matches);
            let message = "Deleted";
            let extra_spacing = 1;

            let general_buffer = display_length / 2 - match display_length % 2 {1 => 2, 0 => 3, _ => 0};
            let spacing = spaces(general_buffer);
            let msg_spacing = spaces(general_buffer - extra_spacing);
            
            println!(
                "\n{}{}\n  {}|\n  {}|\n  {}V",
                msg_spacing,
                message.bright_red(),
                spacing,
                spacing,
                spacing
                );
            display_contacts(&matches);
        },    
        0 => {
            println!("{}", "No contacts found with that name.".bright_red());
        },    
        _ => {
            println!("{}", "There are multiple contacts that match that query:\n".yellow());
            display_contacts(&matches);
            println!("{}", "\nPlease retry with the full name of the contact you wish to delete.".yellow());
        }    
    }    
}

// Exits process if no args supplied
fn exit_if_empty(args: &[String]) {
    if args.len() == 0 {
        println!("{}", "Oops! Remember to enter your argument(s) after the command.".bright_red());
        process::exit(1);
    }
}

pub fn display_contacts(contacts_vec: &Vec<Contact>) {
    if contacts_vec.len() != 0 {
        let length = vec_max_length(&contacts_vec);
        let custom_lb = dashes(&length);
        println!("{}", custom_lb);
        for contact in contacts_vec {
            println!(
                "{}\n{}", 
                contact.display(&length), 
                custom_lb
            );    
        }    
    } else {
        println!("{}", "No matches found.".bright_red());
    }    
}    

pub fn print_docs() {
    let indentation = spaces("contact ".len());
    println!("\n{}", "----------------------------------------------------------------");
    println!("{}\n", "DOCUMENTATION");
    println!("{}", "Available commands:");
    println!("{} {}", "contact".cyan(), "______");
    println!("{}{} [name (can be partial)]", indentation, "search".cyan());
    println!("{}{} [number (can be partial)]", indentation, "reverse-search".cyan());
    println!("{}{} [first name] [last name] [phone number w/ no spaces]", indentation, "add".cyan());
    println!("{}{} [first name] [last name] [phone number w/ no spaces]", indentation, "edit".cyan());
    println!("{}{} [name/part of name]", indentation, "delete".cyan());
    println!("{}{} (displays this documentation)", indentation, "help".cyan());
    println!("\nInitials of commands are valid as shortcuts");
    println!("e.g. 'add' => 'a' and 'reverse-search' => 'rs'");
    println!("{}\n", "----------------------------------------------------------------");
}    

// UTILITY FUNCTION
// Capitalizes Strings
fn capitalize(word: &String) -> String {
    if word.len() == 1 {
        return word.to_uppercase();
    }    
    format!("{}{}", word[..1].to_uppercase(), word[1..].to_lowercase())
}    

// UTILITY FUNCTION
// Returns string of n dashes
fn dashes(n: &usize) -> String {
    let mut line_break = String::new();
    for _i in 0..*n {
        line_break = format!("{}-", line_break);
    }    
    line_break
}    

// UTILITY FUNCTION
// Calculates largest display_length of Contacts in a vector
fn vec_max_length(list: &Vec<Contact>) -> usize {
    let mut max = 0;
    for contact in list {
        if contact.display_length > max {
            max = contact.display_length;
        }    
    }    
    max
}    

// UTILITY FUNCTION
// Generates n spaces
fn spaces(n: usize) -> String {
    let mut result = String::new();
    for _i in 0..n {
        result = format!("{} ", result);
    }    
    result
}    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_spaces() {
        assert_eq!(spaces(5), "     ".to_string());        
        assert_eq!(spaces(2), "  ".to_string());        
        assert_eq!(spaces(17), "                 ".to_string());        
        assert_eq!(spaces(7), "       ".to_string());        
        assert_eq!(spaces(12), "            ".to_string());        
    }

    #[test]
    fn generates_lb() {
        assert_eq!(dashes(&5), "-----".to_string());        
        assert_eq!(dashes(&2), "--".to_string());        
        assert_eq!(dashes(&17), "-----------------".to_string());        
        assert_eq!(dashes(&7), "-------".to_string());        
        assert_eq!(dashes(&12), "------------".to_string());        
    }

    #[test]
    fn capitalizes() {
        assert_eq!(capitalize(&"peacock".to_string()), "Peacock".to_string());
        assert_eq!(capitalize(&"peAcoCK".to_string()), "Peacock".to_string());
        assert_eq!(capitalize(&"PEACOCK".to_string()), "Peacock".to_string());
        assert_eq!(capitalize(&"p".to_string()), "P".to_string());
        assert_eq!(capitalize(&"alfred peacock".to_string()), "Alfred peacock".to_string());
    }
}
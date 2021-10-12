use std::{ fmt, fs, io, process, cmp };

// Every command that accesses a contact should use this struct
// Defines Contact struct
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
}

// Implements Display for use in viewing contact
impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Name: {} {}\nNumber: ({}) {}-{}",
            &self.first,
            &self.last,
            &self.number[..3],
            &self.number[3..6],
            &self.number[6..]
        )
    }
}

// Adds new contact to directory
pub fn add(args: &[String], contents: &String) -> Result<(), io::Error> {
    
    // PLACEHOLDER FOR VALIDATION LOGIC
    //
    //
    
    let new_contact = Contact::new(
        args[0].clone(),
        args[1].clone(),
        args[2].clone()
    );
    
    fs::write("directory.txt", format!(
        "{}{}{} {} {}",
        contents, 
        match contents.as_str() {
            "" => "",
            _ => "\n"
        },
        new_contact.first,
        new_contact.last,
        new_contact.number
    ))?;

    Ok(())
}

// Searches directory for contact information to print
pub fn search(args: &[String], contents: &String) -> Vec<Contact> {
    let mut matches = Vec::new();
    let search_term = match args.len() {
        1 => args[0].clone(),
        2 => format!("{} {}", args[0], args[1]),
        _ => {
            eprintln!("Please enter either a name or a number to search");
            process::exit(1);
        }
    };

    for line in contents.lines() {
        if line.to_lowercase().contains(&search_term.to_lowercase()) {
            let words: Vec<&str> = line.split_whitespace().collect();
            matches.push(Contact::new(
                words[0].to_string(),
                words[1].to_string(),
                words[2].to_string()
            ));
        }
    }

    matches
}


// LOCAL UTILITY FUNCTION
// Capitalizes Strings
fn capitalize(word: &String) -> String {
    format!("{}{}", word[..1].to_uppercase(), word[1..].to_lowercase())
}

// UTILITY FUNCTION
// Returns line break of size n
pub fn lb(n: usize) -> String {
    let mut line_break = String::new();
    for _i in 0..n {
        line_break = format!("{}-", line_break);
    }
    line_break
}

// UTILITY FUNCTION
pub fn vec_max_length(list: &Vec<Contact>) -> usize {
    let mut max = 0;
    for contact in list {
        if contact.display_length > max {
            max = contact.display_length;
        }
    }
    max
}

#[cfg(tests)]
mod tests {

}
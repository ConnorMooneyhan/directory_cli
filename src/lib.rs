use std::{ fmt, fs, io };

// Every command that accesses a contact should use this struct
// Defines Contact struct
struct Contact {
    first: String,
    last: String,
    number: String,
}

impl Contact {
    // Creates new Contact from data
    pub fn new(mut first: String, mut last: String, number: String) -> Contact {
        // Formats input
        first = capitalize(&first.trim().to_string());
        last = capitalize(&last.trim().to_string());

        // Returns new Contact
        Contact {
            first, 
            last,
            number 
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

// LOCAL UTILITY FUNCTION
// Capitalizes Strings
fn capitalize(word: &String) -> String {
    format!("{}{}", word[..1].to_uppercase(), word[1..].to_lowercase())
}

#[cfg(tests)]
mod tests {
    fn create_contact() {

    }
}
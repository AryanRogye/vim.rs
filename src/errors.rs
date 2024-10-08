use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ArgumentError;

// So My First Time Doing Something like this ig this is used to display the errors much better
impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number of arguments")
    }
}

// Error For the ArgumentError
impl Error for ArgumentError {}

pub fn check_args(args : Vec<String> ) -> Result<(), Box<dyn Error>> {
    if args.len() > 2 {
        return Err(Box::new(ArgumentError))
    }
    Ok(())
}

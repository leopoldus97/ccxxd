use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
};

pub mod hex;
pub mod utils;

pub fn get_input(input: Option<String>) -> Result<Box<dyn Read>, Box<dyn Error>> {
    match input {
        Some(file) => {
            let file = File::open(file)?;
            let reader = Box::new(file);
            Ok(reader)
        }
        None => {
            let stdin = io::stdin();
            let reader = Box::new(stdin);
            Ok(reader)
        }
    }
}

pub fn get_output(output: Option<String>) -> Result<Box<dyn Write>, Box<dyn Error>> {
    match output {
        Some(file) => {
            let file = File::create(file)?;
            let writer = Box::new(file);
            Ok(writer)
        }
        None => {
            let stdout = io::stdout();
            let writer = Box::new(stdout);
            Ok(writer)
        }
    }
}

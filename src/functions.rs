use crate::error_manager::Errornumber;
use std::io::{stdin, stdout, Write};



pub fn read_input(prompt: &str) -> Result<String, Errornumber> {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().map_err(Errornumber::from)?;
    stdin().read_line(&mut input).map_err(Errornumber::from)?;
    Ok(input.trim().to_string())
}

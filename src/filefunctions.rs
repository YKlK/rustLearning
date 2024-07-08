
use std::io::Write;
use std::{collections::VecDeque, fs::File};
use std::env;

use serde::{Deserialize, Serialize};
use crate::{error_manager::Errornumber, functions::read_input, student_crud::Students};

 


pub fn create_and_save_students(students_vec: &mut VecDeque<Students>) -> Result<(), Errornumber> {
    let nameofdoc = read_input("Name of the document: ")?;
    
    let mut file = File::create(format!("{}.json", nameofdoc)).map_err(Errornumber::from)?;

    let mut serialize: Vec<String> = Vec::new();
    for student in students_vec.iter() {
        serialize.push(serde_json::to_string(&student).unwrap());
    }

    file.write_all(b"{\"students\":[")?;
    for (i, student) in serialize.iter().enumerate() {
        if i > 0 {
            file.write_all(b",")?;
        }
        file.write_all(student.as_bytes())?;
    }
    file.write_all(b"]}")?;

    println!("JSON file created successfully.");
    Ok(())
}
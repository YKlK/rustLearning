use std::io::Write;
use std::{collections::VecDeque, fs::File};
use std::env;
use serde::{Deserialize, Serialize};
use crate::{error_manager::Errornumber, functions::read_input, student_crud::Students};

 

 
pub fn create_and_save_students(students_vec: &mut VecDeque<Students>) -> Result<(), Errornumber> {
    let nameofdoc = read_input("Name of the document: ")?;
    let typeofdoc =
        read_input("Write the type of document\n\nJson: json\nPDF: pdf\nExcel: xlsx\n\n::")?;

    

    let mut _create_file =
        File::create(format!("{}.{}", nameofdoc, typeofdoc)).map_err(Errornumber::from)?;

    match typeofdoc.trim().to_lowercase().as_str() {
        "xlsx" => {

            // Lógica para manejar excel
            println!("XLSX handling not implemented yet.");
        }
        "json" => {
            let mut serialize:Vec<String>= Vec::new();
            for i in students_vec.into_iter(){
                serialize.push(serde_json::to_string(&i).unwrap());
            }
            _create_file.write_all(b"{\"students\":[")?;
            for i in serialize{
                for y in i.as_bytes(){
                    _create_file.write_all(&[*y])?;
                }
                _create_file.write_all(b",")?;
        }
        // _create_file.
        _create_file.write_all(b"]}")?;

            // Lógica para manejar JSON
            println!("JSON handling not implemented yet.");
        }
        "pdf" => {
            // Lógica para manejar PDF
            println!("PDF handling not implemented yet.");
        }
        _ => {
            println!("Unsupported document type.");
        }
    }
    Ok(())
}

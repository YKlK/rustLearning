use crate::error_manager::Errornumber;
use functions::read_input;
use std::collections::VecDeque;
use student_CRUD::{remove_student, show_students, Students};

mod error_manager;
mod functions;
mod student_CRUD;
fn main() -> Result<(), Errornumber> {
    let mut students_vec: VecDeque<Students> = VecDeque::new();

    loop {
        println!("Options:");
        println!("1. Add student");
        println!("2. Show students");
        println!("3. Update student");
        println!("4. Remove student");
        println!("5. Exit");
        let choice = read_input("Choose an option: ")?;

        match choice.as_str() {
            "1" => match Students::add_student() {
                Ok(student) => students_vec.push_back(student),
                Err(e) => println!("Error: {}", e),
            },
            "2" => show_students(&students_vec)?,
            "3" => {
                if students_vec.is_empty() {
                    println!("No students to update.");
                } else {
                    let id_input = read_input("Enter the ID of the student to update: ")?;
                    {
                        if let Ok(id) = id_input.parse::<u32>() {
                            if let Some(student) = students_vec.iter_mut().find(|s| s.id == id) {
                                if let Err(e) = Students::update_student(student) {
                                    println!("Error: {}", e);
                                }
                            } else {
                                println!("Student with ID {} not found.", id);
                            }
                        } else {
                            println!("Invalid ID.");
                        }
                    }
                }
            }
            "4" => remove_student(&mut students_vec)?,
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}

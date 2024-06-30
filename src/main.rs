use rand::prelude::*;
use std::default::Default;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{stdin, stdout, Error, Write};
use std::fs;
use std::{collections::VecDeque, io};
use std::convert::From;

#[derive(Debug)]
enum Errornumber {
    ErrorNotNumber(String),
    IOError(Error),
}

impl Display for Errornumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Errornumber::IOError(a) => write!(f, "there was an error in the system: {a}"),
            Errornumber::ErrorNotNumber(a) => write!(f, "you have to give us a number: {a}"),
        }
    }
}

impl std::error::Error for Errornumber {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Errornumber::IOError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Errornumber {
    fn from(value: std::io::Error) -> Self {
        Errornumber::IOError(value)
    }
}

#[derive(Debug)]
struct Students {
    id: u32,
    name: String,
    age: u8,
    courses: Vec<String>,
}

impl Default for Students {
    fn default() -> Self {
        Students {
            id: 1,
            name: String::from("si"),
            age: 0,
            courses: Vec::new(),
        }
    }
}

impl Display for Students {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[\nId of the student: {}\nName of the student: {}\nAge of the student: {}\nCourses of the student: {:?}\n]\n",
            self.id, self.name, self.age, self.courses
        )
    }
}

fn read_input(prompt: &str) -> Result<String, Errornumber> {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().map_err(Errornumber::from)?;
    stdin().read_line(&mut input).map_err(Errornumber::from)?;
    Ok(input.trim().to_string())
}

impl Students {
    fn add_student() -> Result<Self, Errornumber> {
        let mut returned: Students = Default::default();
        returned.name = read_input("Name of the student: ")?;
        let age_input = read_input("Give me the age of the student: ")?;
        returned.age = age_input.parse::<u8>().map_err(|_| Errornumber::ErrorNotNumber(age_input.clone()))?;
        
        let courses_input = read_input("How many classes does the student have? ")?;
        let num_of_courses = courses_input.parse::<u8>().map_err(|_| Errornumber::ErrorNotNumber(courses_input.clone()))?;
        
        for i in 1..=num_of_courses {
            let course_name = read_input(&format!("Enter the name of course {}: ", i))?;
            returned.courses.push(course_name);
        }

        returned.id = rand::thread_rng().gen();
        println!("{}", returned);
        Ok(returned)
    }

    fn update_student(stl: &mut Students) -> Result<(), Errornumber> {
        stl.name = read_input("Type the name of the student: ")?;
        
        let age_input = read_input("Type the age of the student: ")?;
        stl.age = age_input.parse::<u8>().map_err(|_| Errornumber::ErrorNotNumber(age_input.clone()))?;
        
        let want_changes = read_input("Do you want to change the courses? (Y/N): ")?;
        
        if want_changes.eq_ignore_ascii_case("Y") {
            let courses_input = read_input("How many classes does the student have? ")?;
            let num_of_courses = courses_input.parse::<u8>().map_err(|_| Errornumber::ErrorNotNumber(courses_input.clone()))?;
            
            stl.courses.clear();
            for i in 1..=num_of_courses {
                let course_name = read_input(&format!("Enter the name of course {}: ", i))?;
                stl.courses.push(course_name);
            }
        }

        println!("{}", stl);
        Ok(())
    }
}

fn show_students(val: &VecDeque<Students>) {
    let mut name = String::new();
    println!("Write nothing if you want all the students, write the name of the student: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut name).unwrap_or_else(|_| {
        println!("There was an error getting the text");
        0
    });

    name = name.trim().to_string(); 

    if !name.is_empty() {
        for i in val.iter().filter(|x| x.name.contains(&name)) {
            print!("{}", i);
        }
    } else {
        for i in val.iter() {
            print!("{}", i);
        }
    }
}

fn remove_student(val: &mut VecDeque<Students>) {
    let mut aux = String::new();
    print!("write the name of the student that you want to erase: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut aux).unwrap_or_else(|_| {
        println!("there was an error taking the name");
        0
    });

    let aux = aux.trim().to_ascii_lowercase();

    if let Some(pos) = val.iter().position(|x| x.name.to_ascii_lowercase() == aux) {
        val.remove(pos);
        println!("Student {} has been removed.", aux);
    } else {
        println!("Student with name {} was not found.", aux);
    }
}

fn main() -> Result<(),Errornumber> {
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
            "1" => {
                match Students::add_student() {
                    Ok(student) =>  students_vec.push_back(student),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => show_students(&students_vec),
            "3" => {
                if students_vec.is_empty() {
                    println!("No students to update.");
                } else {
                    let id_input = read_input("Enter the ID of the student to update: ")?;
                    {if let Ok(id) = id_input.parse::<u32>() {
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
                }}
            }
            "4" => remove_student(&mut students_vec),
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}


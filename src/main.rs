use rand::prelude::*;
use std::default::Default;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Write;
use std::io::{stdin, stdout};
use std::{collections::VecDeque, io};

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
            name: String::new(),
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

impl Students {
    fn add_student() -> Students {
        let mut returned: Students = Default::default();
        let mut input = String::new();
        let mut inputcourses: String = String::new();
        let mut num_of_courses: u8 = 0;

        print!("Name of the student: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut returned.name).unwrap_or(0);
        returned.name = returned.name.trim().to_string();

        print!("Give me the age of the student: ");
        stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            println!("Error reading age: {}", err);
            0
        });
        returned.age = input.trim().parse::<u8>().unwrap_or_else(|_| {
            println!("Invalid age input. Setting age to default (0).");
            0
        });

        print!("How many classes does the student have? ");
        stdout().flush().unwrap();
        stdin().read_line(&mut inputcourses).unwrap_or_else(|_| {
            println!("Invalid number of courses input. Setting courses to default (0).");
            0
        });

        if let Ok(val) = inputcourses.trim().parse::<u8>() {
            num_of_courses = val;
        } else {
            println!("There was an error parsing the number of courses.");
        }

        for i in 1..=num_of_courses {
            let mut course_name = String::new();
            print!("Enter the name of course {}: ", i);
            stdout().flush().unwrap();
            stdin().read_line(&mut course_name).unwrap_or_else(|_| {
                println!("Error reading course name.");
                0
            });
            returned.courses.push(course_name.trim().to_string());
        }

        returned.id = rand::thread_rng().gen();
        println!("{}", returned);
        returned
    }

    fn update_student(stl: &mut Students) {
        let mut input = String::new();
        let mut want_changes = String::new();
        let mut num_of_courses: u8 = 0;

        stl.name.clear();
        print!("Type the name of the student: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut stl.name).unwrap_or_else(|_| {
            println!("There was an error taking the prompts");
            0
        });
        stl.name = stl.name.trim().to_string();

        // Update age
        input.clear();
        print!("Type the age of the student: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap_or_else(|_| {
            println!("There was an error taking the prompts");
            0
        });
        if let Ok(val) = input.trim().parse::<u8>() {
            stl.age = val;
        }

        // Ask if the user wants to change the courses
        want_changes.clear();
        print!("Do you want to change the courses? (Y/N): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut want_changes).unwrap_or_else(|_| {
            println!("There was an error reading the confirmation");
            0
        });

        match want_changes.chars().next().unwrap_or('\n') {
            'Y' | 'y' => {
                input.clear();
                print!("How many classes does the student have? ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap_or_else(|_| {
                    println!("Invalid number of courses input. Setting courses to default (0).");
                    0
                });

                if let Ok(val) = input.trim().parse::<u8>() {
                    num_of_courses = val;
                } else {
                    println!("There was an error parsing the number of courses.");
                }

                stl.courses.clear();
                for i in 1..=num_of_courses {
                    let mut course_name = String::new();
                    print!("Enter the name of course {}: ", i);
                    stdout().flush().unwrap();
                    stdin().read_line(&mut course_name).unwrap_or_else(|_| {
                        println!("Error reading course name.");
                        0
                    });
                    stl.courses.push(course_name.trim().to_string());
                }
            }
            _ => (),
        }

        println!("{}", stl);
    }
}

// fn show_students(val: VecDeque<Students>) ->Vec<Students>{
//     let mut name: String;
//     println!("write nothing if you want all the students, write the name of ");
//     stdin().read_line(&mut name).unwrap_or_else(|_| {
//         print!("there was an error getting the text");
//         0
//     })
//     val.iter().map()

// }

fn main() {
    let mut students_vec: VecDeque<Students> = VecDeque::new();
    let student = Students::default();
}

use std::default::Default;
use std::io::{stdin, stdout};
use std::io::Write;
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

impl Students {
    fn add_student() -> Students {
        let mut returned: Students = Default::default();
        let mut input:String = String::new();
        let mut inputcourses: String = String::new();
        let mut num_of_courses:u8 = 0;
        
        // let trimmer
        print!("Name of the student: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut returned.name).unwrap_or(0);
        print!("give me the age of the student: ");
        stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            println!("Error reading age: {}", err);
            0
        });
        returned.age = input.trim().parse::<u8>().unwrap_or_else(|_| {
            println!("Invalid age input. Setting age to default (0).");
            0
        });

        print!("how many classes does the student have? ");
        stdout().flush().unwrap();
        stdin().read_line(&mut inputcourses).unwrap_or_else(|_| {
            println!("Invalid mount of courses input. Setting age to default (0).");
            0
        });


        if let Ok(val) = inputcourses.trim().parse::<u8>(){
           num_of_courses = val;
        } else{
            print!("there was an error")
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

        print!("{returned:?}",);
        returned 
    }
}
fn main() {
    let mut students_vec: VecDeque<Students> = VecDeque::new();
    let mut student: Students = Default::default();
    
    // loop {}
}

use crate::error_manager::Errornumber;
use crate::functions::read_input;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Students {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub courses: Vec<String>,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
        f,
        "[\nId of the student: {}\nName of the student: {}\nAge of the student: {}\nCourses of the student: {:?}\n]\n",
        self.id, self.name, self.age, self.courses
    )
    }
}
impl Students {
    pub fn add_student() -> Result<Self, Errornumber> {
        let mut returned: Students = Default::default();
        returned.name = read_input("Name of the student: ")?;
        let age_input = read_input("Give me the age of the student: ")?;
        returned.age = age_input
            .parse::<u8>()
            .map_err(|_| Errornumber::ErrorNotNumber(age_input.clone()))?;

        let courses_input = read_input("How many classes does the student have? ")?;
        let num_of_courses = courses_input
            .parse::<u8>()
            .map_err(|_| Errornumber::ErrorNotNumber(courses_input.clone()))?;

        for i in 1..=num_of_courses {
            let course_name = read_input(&format!("Enter the name of course {}: ", i))?;
            returned.courses.push(course_name);
        }

        returned.id = rand::thread_rng().gen();
        println!("{}", returned);
        Ok(returned)
    }

    pub fn update_student(stl: &mut Students) -> Result<(), Errornumber> {
        stl.name = read_input("Type the name of the student: ")?;

        let age_input = read_input("Type the age of the student: ")?;
        stl.age = age_input
            .parse::<u8>()
            .map_err(|_| Errornumber::ErrorNotNumber(age_input.clone()))?;

        let want_changes = read_input("Do you want to change the courses? (Y/N): ")?;

        if want_changes.eq_ignore_ascii_case("Y") {
            let courses_input = read_input("How many classes does the student have? ")?;
            let num_of_courses = courses_input
                .parse::<u8>()
                .map_err(|_| Errornumber::ErrorNotNumber(courses_input.clone()))?;

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

pub fn remove_student(val: &mut VecDeque<Students>) -> Result<(), Errornumber> {
    let aux = read_input("Write the ID of the student that you want to erase: ")?;

    if let Ok(id) = aux.parse::<u32>() {
        if let Some(pos) = val.iter().position(|x| x.id == id) {
            val.remove(pos);
            println!("Student with ID {} has been removed.", id);
        } else {
            println!("Student with ID {} was not found.", id);
        }
    } else {
        println!("Invalid ID.");
    }

    Ok(())
}

pub fn show_students(val: &VecDeque<Students>) -> Result<(), Errornumber> {
    let name =
        read_input("Write nothing if you want all the students, write the name of the student: ")?;

    if !name.is_empty() {
        for i in val.iter().filter(|x| x.name.contains(&name)) {
            print!("{}", i);
        }
    } else {
        for i in val.iter() {
            print!("{}", i);
        }
    }

    Ok(())
}

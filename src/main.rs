use std::{collections::{ VecDeque}, io};

#[derive(Debug)]
struct Students {
    id: u32,
    name: String,
    age: u8,
    courses: Vec<String>,
}
fn main() {
    let mut Students_Vec:VecDeque<Students> = VecDeque::new();
    let mut student_l = String::new();
    io::stdin().read_line(&mut student_l).unwrap_or_else(|x| {
        println!("{x}");
        0
    });
    println!("{student_l}")
}

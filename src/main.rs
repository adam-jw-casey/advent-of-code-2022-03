use advent_of_code_2022_03::calculate_priority;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let priority = calculate_priority(&contents);
    println!("The priority is: {priority}!");
}

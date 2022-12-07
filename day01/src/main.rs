use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let calories: Vec<i32> = inp_data
        .split("\n\n")
        .map(|x| x.lines()
             .map(|y| y.parse::<i32>().unwrap())
             .sum())
        .collect::<Vec<i32>>();

    println!("{}", calories.iter().max().unwrap());

}

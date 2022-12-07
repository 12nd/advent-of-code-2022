use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let mut calories: Vec<i32> = inp_data
        .split("\n\n")
        .map(|x| x.lines()
             .map(|y| y.parse::<i32>().unwrap())
             .sum())
        .collect::<Vec<i32>>();

    // part 1
    println!("{}", calories.iter().max().unwrap());
    // part 2
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>());

}

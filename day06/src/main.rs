use std::env;
use std::fs;
use itertools::Itertools;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let part1: usize = inp_data
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|x| x.iter().all_unique())
        .map(|x| x + 4)
        .unwrap();

    let part2: usize = inp_data
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|x| x.iter().all_unique())
        .map(|x| x + 14)
        .unwrap();

    println!("{}", part1);
    println!("{}", part2);
}

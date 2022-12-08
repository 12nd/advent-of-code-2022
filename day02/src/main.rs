use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for game in inp_data.trim().lines() {
        let mut items = game.split(" ");
        let a = items.next().unwrap();
        let b = items.next().unwrap();

        // rock / lose
        if b == "X" {
            part1 += 1;
            // scissors
            if a == "C" {
                part1 += 6;
                part2 += 2; // paper
            // rock
            } else if a == "A" {
                part1 += 3;
                part2 += 3; // scissors
            } else {
                part2 += 1; // rock
            }

        // paper / draw
        } else if b == "Y" {
            part1 += 2;
            part2 += 3;
            // rock
            if a == "A" {
                part1 += 6;
                part2 += 1; // rock
            // paper
            } else if a == "B" {
                part1 += 3;
                part2 += 2; // paper
            } else {
                part2 += 3; // scissors
            }
        
        // scissors / win
        } else if b == "Z" {
            part1 += 3;
            part2 += 6;
            // paper
            if a == "B" {
                part1 += 6;
                part2 += 3; // scissors
            // scissors
            } else if a == "C" {
                part1 += 3;
                part2 += 1; // rock
            } else {
                part2 += 2; // paper
            }
        }
    }

    println!("{}\n{}", part1, part2);

}

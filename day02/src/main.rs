use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let mut part1: i32 = 0;

    for game in inp_data.trim().lines() {
        let mut items = game.split(" ");
        let a = items.next().unwrap();
        let b = items.next().unwrap();

        if b == "X" {
            part1 += 1;
            if a == "C" {
                part1 += 6;
            } else if a == "A" {
                part1 += 3;
            }
        } else if b == "Y" {
            part1 += 2;
            if a == "A" {
                part1 += 6;
            } else if a == "B" {
                part1 += 3;
            }
        } else if b == "Z" {
            part1 += 3;
            if a == "B" {
                part1 += 6;
            } else if a == "C" {
                part1 += 3;
            }
        }
    }

    println!("{}", part1)

}

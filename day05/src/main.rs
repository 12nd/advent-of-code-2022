use std::env;
use std::fs;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let mut crates: Vec<VecDeque<char>> = Vec::new();

    let mut n: usize = 0;


    // parse initial crate layout
    for l in inp_data.lines() {
        if l.chars().nth(1).unwrap() == '1'{
            break
        }
        for (j, c) in l.chars().skip(1).step_by(4).enumerate() {
            // add column if it does not exist
            if crates.len() == j {
                crates.push(VecDeque::new());
            }
            // if there exists a crate
            if c != ' ' {
                crates[j].push_front(c);
            }
        }
        n += 1;
    }

    // read and perform instructions
    for l in inp_data.trim().lines().skip(n + 2) {
        let instr: Vec<usize> = l.split(" ").skip(1).step_by(2).map(|x| x.parse::<usize>().unwrap()).collect();

        for _ in 0..instr[0] {
            let x = crates[instr[1] - 1].pop_back();
            crates[instr[2] - 1].push_back(x.unwrap());
        }
    }

    // print answer
    for i in crates.iter() {
        print!("{}", i.back().unwrap());
    }
    println!();

}

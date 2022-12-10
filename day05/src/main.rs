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

    // clone crates for part 2
    let mut part2: Vec<VecDeque<char>> = crates.clone();

    // read and perform instructions
    for l in inp_data.trim().lines().skip(n + 2) {
        let instr: Vec<usize> = l.split(" ").skip(1).step_by(2).map(|x| x.parse::<usize>().unwrap()).collect();

        let mut v: VecDeque<char> = VecDeque::new();

        for _ in 0..instr[0] {
            // part 1
            let x = crates[instr[1] - 1].pop_back();
            crates[instr[2] - 1].push_back(x.unwrap());

            // part 2
            let y = part2[instr[1] - 1].pop_back();
            v.push_front(y.unwrap());
        }

        // part 2 append vector
        part2[instr[2] - 1].append(&mut v);
        
    }

    // print answer

    println!("{}", crates.iter().map(|x| x.back().unwrap()).collect::<String>());
    println!("{}", part2.iter().map(|x| x.back().unwrap()).collect::<String>());

}

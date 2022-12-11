use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    // stack containing the full working path
    let mut path_s: Vec<&str> = Vec::new();

    // hashmap of path and sizes
    let mut folders: HashMap<String, usize> = HashMap::new();

    for line in inp_data.trim().lines() {
        let l: Vec<&str> = line.split(" ").collect();

        if l[0] == "$" {
            if l[1] == "cd" {
                if l[2] == ".." {
                    // pop from stack if going back from directory
                    path_s.pop();
                } else {
                    // cd into directory means push to stack
                    path_s.push(l[2])
                }
            }
        } else if l[0] != "dir" {
            // full path string
            let mut fpath: String = String::new();
            for p in path_s.iter() {
                // increase size of all previous directories
                fpath = format!("{}{}/", fpath, p.to_string());
                *folders.entry(fpath.clone()).or_insert(0) += l[0].parse::<usize>().unwrap();
            }
        }
    }

    // part 1
    println!("{}", folders.values().filter(|x| *x <= &100000).sum::<usize>());
}

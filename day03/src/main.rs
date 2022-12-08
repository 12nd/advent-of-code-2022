use std::env;
use std::fs;

fn in_both(a: &str, b: &str) -> usize{
    for x in a.chars() {
        for y in b.chars() {
            if x == y {
                // if lowercase
                if x.is_lowercase(){
                    return (x as u8 - b'a' + 1) as usize;
                } else {
                    return (x as u8 - b'A' + 27) as usize;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let part1: usize = inp_data
        .lines()
        .map(|l| in_both(&l[..l.len()/2], &l[l.len()/2..]))
        .sum();


    println!("{}", part1)

}

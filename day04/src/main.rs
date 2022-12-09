use std::env;
use std::fs;

fn pair_contains(x: Vec<usize>) -> bool {
    // return a >= c && b <= d || c >= a  && d <= b;
    return (x[0] >= x[2] && x[1] <= x[3]) || (x[2] >= x[0] && x[3] <= x[1]);
}

fn pair_overlaps(x: Vec<usize>) -> bool {
    // return a <= d && b >= c || d <= a  && c >= b;
    return (x[0] <= x[3] && x[1] >= x[2]) || (x[3] <= x[0] && x[2] >= x[1]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");
    
    let part1: usize = inp_data
        .trim()
        .lines()
        .map(|x| x.split([',','-'].as_ref()) // split by both delimiters
             .map(|y| y.parse::<usize>().unwrap()) // convert string to integer
             .collect()) // collect into a vector
        .map(|z| pair_contains(z) as usize)
        .sum();

    let part2: usize = inp_data
        .trim()
        .lines()
        .map(|x| x.split([',','-'].as_ref()) // split by both delimiters
             .map(|y| y.parse::<usize>().unwrap()) // convert string to integer
             .collect()) // collect into a vector
        .map(|z| pair_overlaps(z) as usize)
        .sum();

    println!("{}", part1);
    println!("{}", part2);
}

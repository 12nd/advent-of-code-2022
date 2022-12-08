use std::env;
use std::fs;

fn in_both(a: &[u8], b: &[u8]) -> Vec<u8>{
    let mut out: Vec<u8> = Vec::new();
    for x in a {
        for y in b {
            if x == y {
                out.push(*x);
            }
        }
    }
    return out;
}

fn prio(x: u8) -> usize{
    // if lowercase
    if (b'a'..b'z'+1).contains(&x) {
        return (x - b'a' + 1) as usize;
    } else {
        return (x - b'A' + 27) as usize;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let inp_data = fs::read_to_string(&args[1]).expect("Error reading input file");

    let part1: usize = inp_data
        .trim()
        .lines()
        .map(|l| prio(in_both(&l[..l.len()/2].as_bytes(), &l[l.len()/2..].as_bytes())[0]))
        .sum();

    let part2: usize = inp_data
        .trim()
        .lines()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| prio(in_both(x[0], &in_both(x[1],x[2]))[0]))
        .sum();

    println!("{}", part1);
    println!("{}", part2);

}

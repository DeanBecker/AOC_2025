use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_bigint::BigUint;

fn main() {
    println!("AOC: Day 2");

    let mut parsed_ranges: Vec<(u64, u64)> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            let ranges = line.split(",");

            for range in ranges {
                let range_components = range.split("-").map(|c| c.parse::<u64>().expect("Whoops")).collect::<Vec<u64>>();
                parsed_ranges.push((range_components[0], range_components[1]));
            }   
        }
    }

    let mut result = BigUint::ZERO;
    for r in parsed_ranges {
        for i in r.0..=r.1 {
            let s_i = i.to_string();
            let i_len = s_i.len();
            if i_len % 2 == 1 { continue; };

            let first_half = &s_i[0..i_len/2];
            let second_half = &s_i[i_len/2..];

            if first_half == second_half { result += i; };
        }
    }

    println!("Result is {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

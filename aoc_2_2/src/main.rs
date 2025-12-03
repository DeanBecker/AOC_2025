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
        // println!("{0} -> {1}", r.0, r.1);
        for i in r.0..=r.1 {
            // println!("{i}\nFind replicants:");
            let s_i = i.to_string();
            
            let mut j = 1;
            'find_replicate: loop {
                let pattern = &s_i[0..j];
                let s_remainder = &s_i[j..];

                // println!("Pattern: {pattern} in {s_remainder}");

                let mut ix = 0;
                while ix+pattern.len() <= s_remainder.len() {
                    let remaining_substr = s_remainder.len() % pattern.len();
                    if remaining_substr > 0 { break; };

                    ix += pattern.len();
                    let substr = &s_remainder[ix-pattern.len()..ix];
                    // println!("{ix}: {substr}");
                    if substr != pattern { break; };

                    if ix+pattern.len() > s_remainder.len() { 
                        // println!("⭐ Match");
                        result += i;
                        break 'find_replicate;
                    }
                }

                j += 1;
                if j > s_i.len()/2 { break; };
            }
        }
    }

    println!("Result is {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

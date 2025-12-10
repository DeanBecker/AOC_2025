use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn main() {
    println!("AOC: Day 5");

    let mut fresh_produce = Vec::new();
    let mut available_fresh_produce = HashSet::new();

    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut swap_data_section = false;
        for line in lines.map_while(Result::ok) {
            if line.trim() == "" {
                swap_data_section = true;
                continue;
            }

            if !swap_data_section {
                // Fresh produce
                let components = line.split("-").map(|s| s.parse().expect("whoops")).collect::<Vec<u64>>();
                fresh_produce.push((components[0], components[1]));
                
            } else {
                // Available produce
                if let Ok(produce) = line.parse::<u64>() {
                    for (lower_bound, upper_bound) in &fresh_produce {
                        if produce >= *lower_bound && produce <= *upper_bound {
                            available_fresh_produce.insert(produce);
                            continue;
                        }
                    }
                }
            }
        }
    }

    println!("# of fresh ingredients: {}", available_fresh_produce.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

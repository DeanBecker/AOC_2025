use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use aoc_5_1::dsa::stack::{Stack};

fn main() {
    println!("AOC: Day 5");

    let mut intervals = Vec::new();

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            if line.trim() == "" {
                break;
            }

            // Fresh produce
            let components = line
                .split("-")
                .map(|s| s.parse().expect("whoops"))
                .collect::<Vec<u64>>();

            intervals.push((components[0], components[1]));
        }
    }

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merge_stack = Stack::new();

    for interval in intervals {
        if merge_stack.size() == 0 {
            merge_stack.push(interval);
            continue;
        }

        let mut merging_interval = merge_stack.peek().expect("Stack empty?");
        if interval.0 > merging_interval.1 {
            merge_stack.push(interval);
            continue;
        }

        if interval.1 > merging_interval.1 { 
            let new_interval = (merging_interval.0, interval.1);
            _ = merge_stack.pop();
            merge_stack.push(new_interval);
        }
    }
    
    merge_stack.print();
    
    let mut result: u64 = 0;
    while let Some(interval) = merge_stack.pop() {
        result += interval.1 - interval.0 + 1;
    }

    println!("Total fresh ids: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

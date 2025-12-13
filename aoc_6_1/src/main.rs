use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use aoc_6_1::dsa::stack::Stack;

enum Operator {
    Add,
    Mult
}

fn main() {
    println!("AOC: Day 6");

    let mut rows = Vec::new();
    let mut problem_width = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            let line_entries: Vec<String> = line
                .split(' ')
                .map(|x| x.trim().to_owned())
                .filter(|x| x != "")
                .collect();

            if problem_width == 0 {
                problem_width = line_entries.len();
            };

            println!("Line: {:?}", line_entries);

            rows.push(line_entries);
        }
    }

    let mut result: u64 = 0;
    for i in 0..problem_width {
        let mut calc_components = Stack::new(); 
        for r in &rows {
            calc_components.push(r[i].clone());
        }

        let mut operator = Operator::Add;
        if calc_components.pop().expect("No Operator found.") == "*" { operator = Operator::Mult; };

        let mut x: u64 = calc_components.pop().expect("No components found.").parse().expect("Not a number?");
        while calc_components.size() > 0 {
            let next_component: u64 = calc_components.pop().expect("??").parse().expect("Not a number?");
            match operator {
                Operator::Add => x += next_component,
                Operator::Mult => x *= next_component
            };
        }

        result += x;
    }

    println!("Total result {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

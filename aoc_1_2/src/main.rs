use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use finitely::make_ring;

fn main() {
    println!("AOC: Day 1");

    let mut dial_moves: Vec<(i8, u64)> = Vec::new();

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            let direction = if line.as_bytes()[0] == b'L' { -1 } else { 1 }; 
            let magnitude = line[1..].trim();
            let magnitude: u64 = magnitude.parse().expect("Found non-integer movement");
            dial_moves.push((direction, magnitude));
        }
    }


    make_ring! {
        pub Field25 = { Z % 100, x^1 = [2] };
    }

    let mut middle_coeff = Field25::from_int(50);

    let mut password = 0;

    for (dir, m) in dial_moves {
        for _ in (0..m).rev() {
            middle_coeff = if dir == -1 { middle_coeff - 1 } else { middle_coeff + 1 };

            if middle_coeff == 0 {
                password += 1;
            }
        }
    }

    println!("Password is {password}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

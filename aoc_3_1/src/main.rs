use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("AOC: Day 3");

    let mut banks: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            banks.push(line);
        }
    }

    let mut result = 0;
    for bank in banks {
        let parsed_bank: Vec<u32> = bank.chars().map(|c| c.to_digit(10).expect("Whoops")).collect();
        let bank_len = parsed_bank.len();

        let mut largest_ten = 0;
        let mut largest_one = bank_len - 1;

        for i in 1..bank_len-1 {
            if parsed_bank[i] > parsed_bank[largest_ten] { largest_ten = i; };
        }

        for i in (largest_ten+1..bank_len-1).rev() {
            if parsed_bank[i] > parsed_bank[largest_one] { largest_one = i };
        }

        let joltage = parsed_bank[largest_ten] * 10 + parsed_bank[largest_one];
        result += joltage;
    }

    println!("Combined joltage is {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

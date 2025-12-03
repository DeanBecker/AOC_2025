use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_bigint::BigUint;


fn main() {
    println!("AOC: Day 3");
    const TARGET_LEN: usize = 12;

    let mut banks: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            banks.push(line);
        }
    }

    let mut result = BigUint::ZERO;
    for bank in banks {
        println!("{bank}");

        let parsed_bank: Vec<usize> = bank.chars().map(|c| c.to_digit(10).expect("Whoops") as usize).collect();
        let bank_len = parsed_bank.len();
        let mut buffer = bank_len - TARGET_LEN;

        let mut window_start = 0;
        let mut largest: Vec<usize> = Vec::new();
        while largest.len() < TARGET_LEN {
            let window_end = (window_start+buffer).min(bank_len - 1);
            let window = parsed_bank[window_start..=window_end].iter().collect::<Vec<&usize>>();

            let mut window_max = 0;
            let mut new_window_start = window_start;
            let mut new_buffer = buffer;
            for i in 0..window.len() {
                if *window[i] > window_max {
                    window_max = *window[i];
                    new_buffer = buffer - i;
                    new_window_start = window_start + i + 1;
                }
            }
            buffer = new_buffer;
            window_start = new_window_start;
            largest.push(window_max);
        }

        let mut joltage = BigUint::ZERO;
        for i in 0..largest.len() {
            let exponent = (i as i32) - (largest.len() as i32 - 1);
            let exponent = num::abs(exponent);
            joltage += largest[i] * (10_usize.pow(exponent as u32));
        }

        println!("Joltage: {joltage}\n");
        result += joltage;
    }

    println!("Combined joltage is {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

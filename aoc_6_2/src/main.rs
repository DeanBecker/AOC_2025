use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use aoc_6_1::dsa::stack::Stack;
use nalgebra::{Dyn, OMatrix};

enum Operator {
    Add,
    Mult,
}

type DMatrixi32 = OMatrix<i32, Dyn, Dyn>;

fn get_offset_arr(operation_str: &String) -> Vec<usize> {
    let mut offset_arr = Vec::new();
    let str_bytes = operation_str.as_bytes();

    let mut padding = 0;
    for i in 1..str_bytes.len() {
        let b = str_bytes[i];
        match b {
            b' ' => padding += 1,
            b'*' | b'+' => {
                offset_arr.push(padding);
                padding = 0;
            },
            _ => todo!()
        }
    }
    offset_arr.push(padding);

    offset_arr
}

fn main() {
    println!("AOC: Day 6");

    let mut raw_lines = Vec::new();
    let mut rows = Vec::new();
    let mut problem_width = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            raw_lines.push(line.clone());
        }
    }

    let operation_row = &raw_lines.last().expect("Parse went wrong?");
    let offset_arr: Vec<usize> = get_offset_arr(operation_row);

    for line_ix in 0..raw_lines.len() {
        let line = &raw_lines[line_ix];

        let mut line_entries = Vec::new();
        let mut prev_offset = 0;
        let mut delim = 0;
        for offset in &offset_arr {
            let substr = &line[prev_offset+delim..*offset+prev_offset+delim];
            let substr = substr.replace(" ", "$");
            
            line_entries.push(substr);

            prev_offset += *offset + delim;
            delim = 1;
        }

        if problem_width == 0 {
            problem_width = line_entries.len();
        };

        rows.push(line_entries);
    }

    let mut result: u64 = 0;
    for i in 0..problem_width {
        let mut calc_components = Stack::new();
        let mut max_component_len = 0;

        for r in &rows {
            if r[i].len() > max_component_len {
                max_component_len = r[i].len();
            };
            calc_components.push(r[i].clone());
        }

        let mut operator = Operator::Add;
        if calc_components.pop().expect("No Operator found.").contains("*") {
            operator = Operator::Mult;
        };

        let mut storage_mat = DMatrixi32::repeat(calc_components.size(), max_component_len, -1);

        for j in (0..calc_components.size()).rev() {
            let component = calc_components.pop().expect("No components found.");

            let chars = component.as_bytes();
            let offset = max_component_len - chars.len();
            for k in 0..chars.len() {
                let mat_c_ix = k + offset;
                
                let component_str = str::from_utf8(&[chars[k]]).expect("?").to_owned();
                let component_str = component_str.replace("$", "-1");

                let component_integer: i32 = component_str.parse().expect("Not a number?");
                storage_mat[(j, mat_c_ix)] = component_integer;
            }
        }

        let mut col_result = match operator {
            Operator::Add => 0,
            Operator::Mult => 1
        };

        for j in 0..max_component_len {
            let col = storage_mat.column(j);

            let mut component_str = String::new();
            for something in col {
                if *something == -1 { continue; };

                component_str.push_str(&something.to_string());
            }
            let component_int: u64 = component_str.parse().expect("We messed up...");

            match operator {
                Operator::Add => col_result += component_int,
                Operator::Mult => col_result *= component_int
            };
        }

        result += col_result;
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

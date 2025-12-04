use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum GridItem {
    Empty,
    Roll
}

fn main() {
    println!("AOC: Day 4");

    let mut rows: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines.map_while(Result::ok) {
            rows.push(line);
        }
    }

    let grid_width = rows[0].len();
    let grid_height = rows.len();
    let mut grid = build_grid(grid_width, grid_height);
    populate_grid(&mut grid, rows);

    println!("Grid:");
    for r in &grid {
        for c in r {
            print!("{:#?}", c);
        }
        print!("\n");
    }

    println!("\nSolution:");
    let mut total_moveable = 0;
    let mut moveable_rolls = 0;
    while moveable_rolls != -1 {
        moveable_rolls = 0;

        for r in 0..grid_height {
            for c in 0..grid_width {
                let forklift_access = check_space(&grid, c, r);
                if forklift_access { 
                    grid[r][c] = GridItem::Empty;
                    moveable_rolls += 1;
                    print!("*"); 
                } else { 
                    let space = &grid[r][c];
                    print!("{:#?}", space); 
                };
            }
            print!("\n");
        }
        total_moveable += moveable_rolls;
        if moveable_rolls == 0 { moveable_rolls = -1; };
    }

    println!("\nMoveable rolls: {total_moveable}");
}

fn check_space(grid: &Vec<Vec<GridItem>>, x: usize, y: usize) -> bool {
    let space = &grid[y][x];
    if *space == GridItem::Empty { return false; };

    let mut num_of_rolls = 0;
    for i in -1isize..=1 {
        let iy = (y as isize) + i;
        if iy < 0 { continue; };

        for j in -1isize..=1 {
            if i == 0 && j == 0 { continue; };

            let row = grid.get(iy as usize);
            match row {
                Some(row) => {
                    let jx = (x as isize) + j;
                    if jx < 0 { continue; };

                    let col = row.get(jx as usize);
                    match col {
                        Some(col) => if *col == GridItem::Roll { 
                            num_of_rolls += 1;
                        },
                        None => continue
                    };
                },
                None => continue
            };
        }
    }

    num_of_rolls < 4
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_grid(grid_width: usize, grid_height: usize) -> Vec<Vec<GridItem>> {
    let mut grid: Vec<Vec<GridItem>> = Vec::with_capacity(grid_height);
    for _ in 0..grid_height {
        let row: Vec<GridItem> = Vec::with_capacity(grid_width);
        grid.push(row);
    }

    grid
}

fn populate_grid(grid: &mut Vec<Vec<GridItem>>, rows: Vec<String>) {
    for i in 0..rows.len() {
        let raw_row = &rows[i];
        let row = &mut grid[i];

        for c in raw_row.chars() {
            let grid_value = match c {
                '.' => GridItem::Empty,
                '@' => GridItem::Roll,
                _ => todo!()
            };

           row.push(grid_value); 
        }
    }
}

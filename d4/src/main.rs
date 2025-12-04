use std::{env, fs::File, io::{self, BufRead, BufReader}};
use std::str::FromStr;
use crate::cli_options::CliOptions;

mod cli_options;

const EXAMPLE_FILE_PATH: &str = "example_input.txt";
const ACTUAL_FILE_PATH: &str = "input.txt";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli_options = CliOptions::from_args(&args);

    let file_path = if cli_options.example {
        EXAMPLE_FILE_PATH
    } else {
        ACTUAL_FILE_PATH
    };
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;

    let p1_result = part_1(&lines)?;
    println!("[Part 1]: {}", p1_result);

    let p2_result = part_2(&lines)?;
    println!("[Part 2]: {}", p2_result);

    Ok(())
}

fn part_1(rows: &Vec<String>) -> io::Result<i32> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for row in rows {
        let row_digits = row.chars().map(|c| {
            if c == '@' {
                1
            } else {
                0
            }
        }).collect::<Vec<i32>>();
        grid.push(row_digits);
    }

    let mut accessible_rolls = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                continue;
            }

            let mut neighbors = 0;
            for k in (-1 as isize)..2 {
                for l in (-1 as isize)..2 {
                    let new_i = (i as isize) + k;
                    let new_j = (j as isize) + l;
                    if (new_i, new_j) == (i as isize, j as isize) {
                        continue;
                    }

                    if (new_i >= 0 && (new_i as usize) < grid.len()) && (new_j >= 0 && (new_j as usize) < grid[(new_i as usize)].len()) {
                        if grid[(new_i as usize)][(new_j as usize)] == 1 {
                            neighbors += 1;
                        }
                    }
                }
            }

            if neighbors < 4 {
                //println!("({},{}) = {} ACCESSIBLE", i, j, neighbors);
                accessible_rolls += 1
            } else {
                //println!("({},{}) = {}", i, j, neighbors);
            }
        }
    }

    Ok(accessible_rolls)
}

fn part_2(rows: &Vec<String>) -> io::Result<usize> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for row in rows {
        let row_digits = row.chars().map(|c| {
            if c == '@' {
                1
            } else {
                0
            }
        }).collect::<Vec<i32>>();
        grid.push(row_digits);
    }

    let mut rolls_removed = 0;
    loop {
        let (count, updated_grid) = remove_rolls(grid.clone())?;
        grid = updated_grid;

        if count == 0 {
            break;
        } else {
            rolls_removed += count;
        }
    }

    Ok(rolls_removed)
}

fn remove_rolls(mut grid: Vec<Vec<i32>>) -> io::Result<(usize, Vec<Vec<i32>>)> {
    let mut rolls_to_remove = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                continue;
            }

            let mut neighbors = 0;
            for k in (-1 as isize)..2 {
                for l in (-1 as isize)..2 {
                    let new_i = (i as isize) + k;
                    let new_j = (j as isize) + l;
                    if (new_i, new_j) == (i as isize, j as isize) {
                        continue;
                    }

                    if (new_i >= 0 && (new_i as usize) < grid.len()) && (new_j >= 0 && (new_j as usize) < grid[(new_i as usize)].len()) {
                        if grid[(new_i as usize)][(new_j as usize)] == 1 {
                            neighbors += 1;
                        }
                    }
                }
            }

            if neighbors < 4 {
                //println!("({},{}) = {} ACCESSIBLE", i, j, neighbors);
                rolls_to_remove.push((i, j));
            } else {
                //println!("({},{}) = {}", i, j, neighbors);
            }
        }
    }

    for (i, j) in rolls_to_remove.clone() {
        grid[i][j] = 0;
    }

    Ok((rolls_to_remove.len(), grid))
}
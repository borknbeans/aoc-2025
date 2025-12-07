use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;
use std::{env, fs::File, io::{self, BufRead, BufReader}};
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

fn part_1(lines: &[String]) -> io::Result<i64> {
    let mut grid: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let c = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        grid.push(c);
    }

    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut split_count = 0;

    let (start_row, start_col) = (0, grid[0].iter().position(|x| x == "S").unwrap());

    let mut drop_spots: Vec<(usize, usize)> = vec![(start_row, start_col)];
    while drop_spots.len() != 0 {
        let (row, col) = drop_spots.pop().unwrap();
        //println!("checking drop spot: ({}, {})", row, col);
        for i in row..grid.len() {
            if grid[i][col] == "^" {
                if visited.contains(&(i, col)) {
                    break;
                }
                //println!("split at: ({}, {})", i, col);
                split_count += 1;
                drop_spots.push((i, col - 1));
                drop_spots.push((i, col + 1));
                visited.push((i, col));
                break;
            }
        }
    }

    Ok(split_count)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    let mut grid: Vec<Vec<String>> = Vec::new();

    lines.iter().for_each(|l| {
        grid.push(l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
    });

    let mut map: HashMap<usize, i64> = HashMap::new();
    for i in (0..grid.len() - 1).rev().step_by(2) { // -1 because the last row is just dots and step by 2 because every other is just dots
        let splitter_indices = grid[i].iter().enumerate().filter_map(|(index, value)| {
            if value == "^" {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

        splitter_indices.iter().for_each(|ind| {
            let left: i64 = if let Some(v) = map.get(&(ind - 1)) {
                v.clone()
            } else {
                1
            };

            let right: i64 = if let Some(v) = map.get(&(ind + 1)) {
                v.clone()
            } else {
                1
            };
            map.insert(ind.clone(), left + right);
        });
    }

    let start_col = grid[0].iter().position(|x| x == "S").unwrap();

    Ok(map.get(&start_col).unwrap().clone())
}
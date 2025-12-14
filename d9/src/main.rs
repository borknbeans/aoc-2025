use std::collections::{HashMap, HashSet};
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
    let mut red_tiles: Vec<(i64, i64)> = Vec::new();

    lines.iter().for_each(|l| {
        let nums = l.split(",").map(|s| i64::from_str(s).unwrap()).collect::<Vec<i64>>();
        red_tiles.push((nums[0], nums[1]));
    });


    let mut biggest_area = 0;
    for i in 0..(red_tiles.len() - 1) {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            //println!("({},{}) x ({},{}) = {}", x1,y1,x2,y2,area);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    Ok(biggest_area)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    let mut red_tiles: Vec<(i64, i64)> = Vec::new();

    lines.iter().for_each(|l| {
        let nums = l.split(",").map(|s| i64::from_str(s).unwrap()).collect::<Vec<i64>>();
        red_tiles.push((nums[0], nums[1]));
    });

    let mut biggest_area = 0;
    for i in 0..(red_tiles.len() - 1) {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            //println!("({},{}) x ({},{}) = {}", x1,y1,x2,y2,area);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    Ok(biggest_area)
}
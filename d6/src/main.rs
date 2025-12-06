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
    let mut grid = Vec::new();
    let mut operations = Vec::new();

    for line in lines {
        let nums = line.split(" ").filter_map(|s| i64::from_str(s).ok()).collect::<Vec<i64>>();
        if nums.len() == 0 {
            operations = line.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        }
        grid.push(nums);
    }

    let mut result = 0;
    for i in 0..grid.len() {
        let op = operations[i];
        let mut r = 0;
        if op == "*" {
            r = 1;
        }

        for j in 0..grid[i].len() {
            let v = grid[i][j];
            print!("{} ", v);
            if op == "*" {
                r *= v;
            } else if op == "+" {
                r += v;
            }
        }
        // if op == "*" {
        //     r = col.iter().product();
        // } else if op == "+" {
        //     r = col.iter().sum();
        // }

        // for c in col {
        //     print!("{} ", c);
        // }
        println!("");
        println!("{}", r);
        result += r;
    }

    Ok(result)
}

fn part_2(lines: &[String]) -> io::Result<i32> {
    Ok(0)
}
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

fn part_1(lines: &[String]) -> io::Result<i32> {
    Ok(0)
}

fn part_2(lines: &[String]) -> io::Result<i32> {
    Ok(0)
}
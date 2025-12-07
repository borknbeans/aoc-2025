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
        if nums.is_empty() {
            operations = line.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
            continue;
        }
        grid.push(nums);
    }

    let mut result = 0;
    
    for i in 0..operations.len() {
        let operation = operations[i];

        let mut inner_result = 0;

        if operation == "*" && inner_result == 0 {
            inner_result = 1;
        }
        for j in 0..grid.len() {
            //print!("({}, {}) - ", j, i);
            let num = grid[j][i];

            //println!("{}", num);

            if operation == "*" {
                inner_result *= num;
            } else if operation == "+" {
                inner_result += num;
            }
        }
        //println!("r: {}", inner_result);

        result += inner_result;
        //println!("ACTUAL result: {}", result)
    }

    Ok(result)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    let mut char_grid: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let chars = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        char_grid.push(chars);
    }

    let mut operations: Vec<&str> = Vec::new();
    let mut num_groups: Vec<Vec<i64>> = Vec::new();
    let mut temp_group: Vec<i64> = Vec::new();
    for col in 0..char_grid[0].len() {
        let mut str = "".to_string();
        for row in 0..char_grid.len() {
            let c = &char_grid[row][col];
            if c == "*" || c == "+" {
                operations.push(c);
            } else {
                str += c;
            }
        }

        str = str.trim().to_string();
        if str.is_empty() {
            num_groups.push(temp_group.clone());
            temp_group.clear();
            //println!();
        } else {
            //println!("{}", str);
            let parsed_num = i64::from_str(&str).unwrap();
            temp_group.push(parsed_num);
        }
    }
    num_groups.push(temp_group.clone()); // have to catch the last one

    let mut total = 0;
    for i in 0..operations.len() {
        let operation = operations[i];
        let nums = &num_groups[i];

        let result: i64 = match operation {
            "*" => nums.iter().product(),
            "+" => nums.iter().sum(),
            _ => 0,
        };
        //println!("result: {}", result);
        total += result;
    }

    Ok(total)
}
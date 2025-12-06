use std::collections::HashSet;
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
    let (ids, ranges) = parse_fresh_ids(lines)?;

    let mut count = 0;
    for id in ids {
        let num = i64::from_str(id).unwrap();
        
        for (lower, upper) in ranges.clone() {
            if num >= lower && num <= upper {
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

// This is too slow and needs to be optimized
// I can subtract the lower from the upper and add the count
// HOWEVER, i also need to consider ranges that overlap
// How I do that is TBD

// I need to combine overlapping ranges
// I can check if a lower bound is less than another ranges upper bound
// if so i combine the upper bound and lower bound respectively
fn part_2(lines: &[String]) -> io::Result<i64> {
    let (_, mut ranges) = parse_fresh_ids(lines)?;

    ranges.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    if ranges.is_empty() {
        return Ok(0);
    }

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        if current.1 >= start - 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    let count: i64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    Ok(count)
}

fn parse_fresh_ids(lines: &[String]) -> io::Result<(&[String], Vec<(i64, i64)>)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut split = lines.split(|s| s == "");
    let fresh_id_ranges = split.next().unwrap();
    let ids = split.next().unwrap();

    for range in fresh_id_ranges {
        let mut iter = range.split("-");
        let lower_bound = i64::from_str(iter.next().unwrap()).unwrap();
        let upper_bound = i64::from_str(iter.next().unwrap()).unwrap();

        ranges.push((lower_bound, upper_bound));
    }

    Ok((ids, ranges))
}
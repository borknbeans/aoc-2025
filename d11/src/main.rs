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
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    lines.iter().for_each(|l| {
        let key_value_split = l.split(":").map(|s| s.to_string()).collect::<Vec<String>>();
        let key = key_value_split[0].clone();
        let values = key_value_split[1].split(" ").filter_map(|mut s| {
            s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }).collect::<HashSet<String>>();

        if map.contains_key(&key) {
            if let Some(v) = map.get_mut(&key) {
                v.extend(values.clone());
            }
        }
        map.insert(key.clone(), values);
    });

    let start = &map["you"];

    let mut paths_to_explore: Vec<String> = start.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let mut curr = paths_to_explore.pop();
    let mut valid_paths = 0;
    loop {
        if paths_to_explore.len() == 0 {
            break;
        }

        while curr != Some("out".to_string()) && curr != None {
            let options = &map[curr];
            if options.len() == 0 {
                break;
            }
            paths_to_explore.extend(options.iter().map(|s| s.to_string()).collect::<Vec<String>>()[1..]);
        }
    }


    Ok(0)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    Ok(0)
}
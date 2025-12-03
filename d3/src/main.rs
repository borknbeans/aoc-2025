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

/*
Criteria:
- Each line is a "battery bank" i.e. 987654321111111
- 2 batteries need to be turned on in each bank
  - these batteries when combined give the amount of jolts produced i.e. 987654321111111 -> turning on 8 and 2 -> 82 jolts
- need to find the largest possible jolts for each bank
- cannot rearrange the batteries in the bank
- the answer is the sum of the largest jolts for each bank
*/
fn part_1(lines: &[String]) -> io::Result<i32> {
    let mut jolts_sum = 0;
    for line in lines {
        let mut ratings = line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();

        // Find the 2 largest numbers
        // If there is duplicate numbers
        //    - For the bigger of the two, pick the earliest occurence
        //    - For the smaller of the two, pick the latest occurence
        let largest_num = ratings.iter().max().unwrap();
        let largest_num_index = ratings.iter().position(|x| x == largest_num).unwrap();

        let mut second_largest_num = -1;
        let mut second_largest_num_index = -1;
        for i in (largest_num_index + 1)..ratings.len() {
            if ratings[i] > second_largest_num {
                second_largest_num = ratings[i];
                second_largest_num_index = i as i32;
            }
        }

        if second_largest_num_index == -1 {
            for i in 0..largest_num_index {
                if ratings[i] > second_largest_num {
                    second_largest_num = ratings[i];
                    second_largest_num_index = i as i32;
                }
            }
        }
        let jolts = if second_largest_num_index < largest_num_index as i32 {
            i32::from_str(&format!("{}{}", second_largest_num, largest_num)).unwrap()
        } else {
            i32::from_str(&format!("{}{}", largest_num, second_largest_num)).unwrap()
        };

        jolts_sum += jolts;
        // println!("jolts: {}", jolts);
    }

    Ok(jolts_sum)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    const NUM_DIGITS: u32 = 12;
    let mut jolts_sum = 0;
    for bank in lines {
        let ratings = bank.chars().map(|c| i32::from_str(&c.to_string()).unwrap()).collect::<Vec<i32>>();

        let mut joltage_str = "".to_string();
        let mut idx = 0;
        for n in 1..(NUM_DIGITS + 1) {
            let mut max_num = 0;
            let mut max_num_idx = 0;
            for i in idx..(ratings.len() - (NUM_DIGITS - n) as usize) {
                if ratings[i] > max_num {
                    max_num = ratings[i];
                    max_num_idx = i;
                }
            }
            idx = max_num_idx + 1;
            joltage_str = format!("{}{}", joltage_str, max_num);
        }
        // println!("jolts: {}", joltage_str);
        jolts_sum += i64::from_str(&joltage_str).unwrap()
    }

    Ok(jolts_sum)
}
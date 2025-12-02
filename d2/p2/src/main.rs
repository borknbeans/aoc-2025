use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FILE_PATH: &str = "input.txt";


fn main()  -> io::Result<()> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let mut invalid_id_sum = 0;
    for line in reader.lines() {
        let line = line?;
        let id_ranges = line.split(",").collect::<Vec<&str>>();
        for id_range in id_ranges {
            invalid_id_sum += sum_invalid_ids(id_range)
        }
    }

    println!("Invalid ID sum: {}", invalid_id_sum);

    Ok(())
}

fn sum_invalid_ids(id_range: &str) -> i64 {
    let lower_bound = id_range.split("-").nth(0).unwrap().parse::<i64>().unwrap();
    let upper_bound = id_range.split("-").nth(1).unwrap().parse::<i64>().unwrap();
    
    let mut invalid_id_sum = 0;
    for id in lower_bound..=upper_bound {
        let valid = is_valid_id(&id.to_string());
        if !valid {
            println!("ID {} is {}", id, valid);
            invalid_id_sum += id;
        }

    }
    return invalid_id_sum;
}

fn is_valid_id(id: &str) -> bool {
    let chars = id.chars().collect::<Vec<char>>();
    for substring_len  in 1..chars.len() {
        if chars.len() % substring_len != 0 {
            continue;
        }
        let substrings = chars.chunks(substring_len).collect::<Vec<&[char]>>().iter().map(|s| s.iter().collect::<String>()).collect::<Vec<String>>();

        let mut previous_substring: Option<String> = None;
        let mut valid = false;
        for substring in substrings {
            if let Some(ref prev) = previous_substring {
                if prev != &substring {
                    valid = true;
                    break;
                }
            } else {
                previous_substring = Some(substring);
            }
        }
        if !valid {
            return false
        }
    }

    return true;
}
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
    if id.len() % 2 != 0 {
        return true; // odd length ids are always valid because they can't be repeated exactly twice
    }

    let half_1 = &id[0..id.len()/2];
    let half_2 = &id[id.len()/2..id.len()];

    return half_1 != half_2;
}
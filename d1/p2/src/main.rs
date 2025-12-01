use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

const FILE_PATH: &str = "input.txt";

fn main() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let mut num = DialNumber::new(50);
    for line in reader.lines() {
        let line = line?;
        num = num.parse_input(&line);
    }

    println!("Zero count: {}", num.zero_counter);

    Ok(())
}

struct DialNumber {
    number: i32,
    zero_counter: i32,
}

impl DialNumber {
    pub fn new(number: i32) -> Self {
        Self { number , zero_counter: 0}
    }

    pub fn parse_input(self, input: &str) -> Self {
        let re = Regex::new(r"([LR])(\d+)").unwrap();
        if let Some(caps) = re.captures(input) {
            let direction = &caps[1];
            let steps: i32 = caps[2].parse().unwrap();

            if direction == "L" {
                return self.rotate_left(steps);
            } else if direction == "R" {
                return self.rotate_right(steps);
            }
        }
        
        self
    }

    pub fn rotate_left(self, steps: i32) -> Self {
        let old_number = self.number;
        let mut number = self.number - steps;
        let mut zero_counter = self.zero_counter;
        if number <= 0 {
            let times_passed_zero = (number / 100).abs();
            if old_number != 0 {
                zero_counter = zero_counter + 1;
            }
            number = (100 + (number % 100)) % 100;
            zero_counter = zero_counter + times_passed_zero;
        }
        //println!("[L{}] Rotate left from {} to {} (zero_counter: {})", steps, old_number, number, zero_counter);
        Self { number, zero_counter }
    }

    pub fn rotate_right(self, steps: i32) -> Self {
        let old_number = self.number;
        let mut number = self.number + steps;
        let mut zero_counter = self.zero_counter;
        if number > 99 {
            let times_passed_zero = number / 100;
            zero_counter = zero_counter + times_passed_zero;
            number = number % 100;
        }
        //println!("[R{}] Rotate right from {} to {} (zero_counter: {})", steps, old_number, number, zero_counter);
        Self { number, zero_counter }
    }
}

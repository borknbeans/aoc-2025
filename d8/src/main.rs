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

    let p1_result = part_1(&lines, cli_options.example)?;
    println!("[Part 1]: {}", p1_result);

    let p2_result = part_2(&lines)?;
    println!("[Part 2]: {}", p2_result);

    Ok(())
}

fn part_1(lines: &[String], example: bool) -> io::Result<i64> {
    let mut pos_vec: Vec<Position> = Vec::new();
    for line in lines {
        let pos = line.split(",").map(|s| i64::from_str(s).unwrap()).collect::<Vec<i64>>();
        let v = Position::new(pos[0], pos[1], pos[2]);
        pos_vec.push(v);
    }

    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..(pos_vec.len() - 1) {
        for j in (i + 1)..pos_vec.len() {
            let dist = pos_vec[i].distance_to(pos_vec[j]);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // distances.iter().for_each(|v| {
    //     println!("({},{},{}) <-> ({},{},{}) = {}", pos_vec[v.0].x, pos_vec[v.0].y, pos_vec[v.0].z, pos_vec[v.1].x, pos_vec[v.1].y, pos_vec[v.1].z, v.2)
    // });

    let max_connections = if example {
        10
    } else {
        1000
    };

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut connections = 0;
    for i in 0..distances.len() {
        if connections >= max_connections {
            break;
        }

        let (pos1_ind, pos2_ind, _) = distances[i];

        //print!("({},{},{}) <-> ({},{},{}) => ", pos_vec[pos1_ind].x, pos_vec[pos1_ind].y, pos_vec[pos1_ind].z, pos_vec[pos2_ind].x, pos_vec[pos2_ind].y, pos_vec[pos2_ind].z);

        let mut circuit_ind_for_pos1: Option<usize> = None;
        let mut circuit_ind_for_pos2: Option<usize> = None;
        for j in 0..circuits.len() {
            let circuit = &circuits[j];
            if circuit.contains(&pos1_ind) {
                circuit_ind_for_pos1 = Some(j);
            }
            if circuit.contains(&pos2_ind) {
                circuit_ind_for_pos2 = Some(j);
            }
        }

        if let (Some(ind1), Some(ind2)) = (circuit_ind_for_pos1, circuit_ind_for_pos2) {
            if ind1 == ind2 {
                //println!("already in same circuit");
                connections += 1;
                continue;
            } else {
                // merge the 2 sets
                let merged_set = circuits[ind1].union(&circuits[ind2]).cloned().collect::<HashSet<usize>>();
                circuits[ind1] = merged_set;
                circuits.remove(ind2);
                //println!("need to be merged");
                connections += 1;
            }
        } else if let Some(ind) = circuit_ind_for_pos1 {
            //println!("adding pos2 to pos1 circuit");
            circuits[ind].insert(pos2_ind);
            connections += 1;
        } else if let Some(ind) = circuit_ind_for_pos2 {
            //println!("adding pos1 to pos2 circuit");
            circuits[ind].insert(pos1_ind);
            connections += 1;
        } else {
            //println!("brand new circuit");
            let mut set = HashSet::new();
            set.insert(pos1_ind);
            set.insert(pos2_ind);

            circuits.push(set);
            connections += 1;
        }
        //println!("connections: {}", connections);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    // println!("num of circuits: {}", circuits.len());
    // circuits.iter().for_each(|set| {
    //     print!("len {} : ", set.len());
    //     set.iter().for_each(|v| print!("({},{},{}) ", pos_vec[*v].x, pos_vec[*v].y, pos_vec[*v].z));
    //     println!()
    // });

    let r = (circuits[0].len() * circuits[1].len() * circuits[2].len()) as i64;
    Ok(r)
}

fn part_2(lines: &[String]) -> io::Result<i64> {
    let mut pos_vec: Vec<Position> = Vec::new();
    for line in lines {
        let pos = line.split(",").map(|s| i64::from_str(s).unwrap()).collect::<Vec<i64>>();
        let v = Position::new(pos[0], pos[1], pos[2]);
        pos_vec.push(v);
    }

    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..(pos_vec.len() - 1) {
        for j in (i + 1)..pos_vec.len() {
            let dist = pos_vec[i].distance_to(pos_vec[j]);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for i in 0..pos_vec.len() {
        let mut set = HashSet::new();
        set.insert(i);
        circuits.push(set);
    }

    let mut connections = 0;
    let mut last_connection: (usize, usize) = (0, 0);
    for i in 0..distances.len() {
        if circuits.len() == 1 {

            break;
        }

        let (pos1_ind, pos2_ind, _) = distances[i];
        last_connection = (pos1_ind, pos2_ind);
        //print!("({},{},{}) <-> ({},{},{}) => ", pos_vec[pos1_ind].x, pos_vec[pos1_ind].y, pos_vec[pos1_ind].z, pos_vec[pos2_ind].x, pos_vec[pos2_ind].y, pos_vec[pos2_ind].z);

        let mut circuit_ind_for_pos1: Option<usize> = None;
        let mut circuit_ind_for_pos2: Option<usize> = None;
        for j in 0..circuits.len() {
            let circuit = &circuits[j];
            if circuit.contains(&pos1_ind) {
                circuit_ind_for_pos1 = Some(j);
            }
            if circuit.contains(&pos2_ind) {
                circuit_ind_for_pos2 = Some(j);
            }
        }

        if let (Some(ind1), Some(ind2)) = (circuit_ind_for_pos1, circuit_ind_for_pos2) {
            if ind1 == ind2 {
                //println!("already in same circuit");
                connections += 1;
                continue;
            } else {
                // merge the 2 sets
                let merged_set = circuits[ind1].union(&circuits[ind2]).cloned().collect::<HashSet<usize>>();
                circuits[ind1] = merged_set;
                circuits.remove(ind2);
                //println!("need to be merged");
                connections += 1;
            }
        } else if let Some(ind) = circuit_ind_for_pos1 {
            //println!("adding pos2 to pos1 circuit");
            circuits[ind].insert(pos2_ind);
            connections += 1;
        } else if let Some(ind) = circuit_ind_for_pos2 {
            //println!("adding pos1 to pos2 circuit");
            circuits[ind].insert(pos1_ind);
            connections += 1;
        } else {
            //println!("brand new circuit");
            let mut set = HashSet::new();
            set.insert(pos1_ind);
            set.insert(pos2_ind);

            circuits.push(set);
            connections += 1;
        }
        //println!("connections: {}", connections);
    }
    //print!("({},{},{}) <-> ({},{},{}) => ", pos_vec[last_connection.0].x, pos_vec[last_connection.0].y, pos_vec[last_connection.0].z, pos_vec[last_connection.1].x, pos_vec[last_connection.1].y, pos_vec[last_connection.1].z);

    //circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    // println!("num of circuits: {}", circuits.len());
    // circuits.iter().for_each(|set| {
    //     print!("len {} : ", set.len());
    //     set.iter().for_each(|v| print!("({},{},{}) ", pos_vec[*v].x, pos_vec[*v].y, pos_vec[*v].z));
    //     println!()
    // });

    //let r = (circuits[0].len() * circuits[1].len() * circuits[2].len()) as i64;
    let r = pos_vec[last_connection.0].x * pos_vec[last_connection.1].x;
    Ok(r)
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn distance_to(self, pos: Position) -> f64 {
        let x_dist = (pos.x - self.x).pow(2);
        let y_dist = (pos.y - self.y).pow(2);
        let z_dist = (pos.z - self.z).pow(2);

        ((x_dist + y_dist + z_dist) as f64).sqrt()
    }
}
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
    panic,
};

use itertools::Itertools;

/*
    Day 2

    Problem:
    Track Forward and Depth movements but summing each position. Finally mutltiply the forward
    and depth movements together to get the result.
*/

fn lines_from_file<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let buf = BufReader::new(io);
    buf.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let buf = lines_from_file(File::open("day_02/src/input.txt")?)?;

    let mut parsed_movements: Vec<(&str, u32)> = Vec::new();
    for line in buf.iter() {
        if let Some((l1, l2)) = line.split_whitespace().collect_tuple() {
            parsed_movements.push((l1, l2.parse().unwrap()));
        } else {
            panic!("Expected two elements")
        }
    }

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (direction, movement) in parsed_movements {
        if direction == "forward" {
            horizontal_position += movement;
            depth += aim * movement;
        } else if direction == "down" {
            aim += movement;
        } else if direction == "up" {
            aim -= movement;
        } else {
            panic!(
                "Expected 'forward' or 'down' command but received {}",
                direction
            );
        }
    }

    println!("Horizontal Sum: {}", horizontal_position);
    println!("Depth Sum: {}", depth);
    println!("Horizontal * Depth =  {}", depth * horizontal_position);

    Ok(())
}

mod day_01;
use std::{fs::File, io::Error};

use day_01::{lines_from_file, measure_depth_increase, measure_depth_increase_2};

fn test_day_01() -> Result<(), Error> {
    let input = lines_from_file(File::open("src/day_01_input.txt")?)?;

    println!(
        "Part 1: Count of depth increases: {}",
        measure_depth_increase(&input)
    );

    println!(
        "Part 2: Count of depth increases: {}",
        measure_depth_increase_2(&input)
    );
    measure_depth_increase_2(&input);

    Ok(())
}

fn main() {
    test_day_01().unwrap();
}

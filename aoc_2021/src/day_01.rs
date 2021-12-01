use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
};

/*
    Day 1

    Problem:
    Count how many times the depth measure increases from the previous measurement
*/

pub fn lines_from_file<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let buf = BufReader::new(io);
    buf.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn measure_depth_increase(input: &[i64]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn measure_depth_increase_2(input: &[i64]) -> usize {
    let res: Vec<i64> = input.windows(3).map(|w| w.iter().sum()).collect();
    measure_depth_increase(&res)
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
    panic,
};

/*
    Day 3

    Problem:
    Part 1:
        - Check power consumption of sub
        - Use the binary numbers to generate two new numbers (gamma rate, epsilon rate)
        - Power consumption = gama_rate * epsilon_rate
        - Find the most common bit in the corrosponding position to get gamma rate
        - find the least common bit in each position to get the epsilon rate
        - Provide answer in decimal

    Part 2:

*/

fn lines_from_file<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let buf = BufReader::new(io);
    buf.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_gamma(line_buf: &[String]) -> Vec<usize> {
    let mut gamma_rate: Vec<usize> = Vec::new();

    let mut position = 0;

    'main: loop {
        let mut zeros = 0;
        let mut ones = 0;

        for line in line_buf.iter() {
            let current_read = line.chars().collect::<Vec<char>>()[position];

            if current_read == '0' {
                zeros += 1;
            } else if current_read == '1' {
                ones += 1;
            } else {
                panic!("Expected a 1 or a 0");
            }
        }

        if zeros > ones {
            gamma_rate.push(0);
        } else {
            gamma_rate.push(1);
        }

        if position >= line_buf[0].len() - 1 {
            break 'main;
        };
        position += 1;
    }

    gamma_rate
}

fn binary_to_dec(binary_arr: &[usize]) -> usize {
    binary_arr
        .iter()
        .enumerate()
        .map(|(i, n)| return n * (usize::pow(2, 11_u32 - i as u32)))
        .sum()
}

fn main() -> Result<(), Error> {
    let buf = lines_from_file(File::open("day_03/src/input.txt")?)?;

    let gamma_rate = get_gamma(&buf);

    let epsilon_rate: Vec<usize> = gamma_rate
        .iter()
        .map(|&x| if x == 1 { x - 1 } else { x + 1 })
        .collect();

    let res = binary_to_dec(&gamma_rate) * binary_to_dec(&epsilon_rate);

    println!("Gamma Rate: {}", binary_to_dec(&gamma_rate));
    println!("Epsilon Rate: {}", binary_to_dec(&epsilon_rate));
    println!("Power consumption = {}", res);

    Ok(())
}

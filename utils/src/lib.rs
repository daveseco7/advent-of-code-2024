use std::fs::read_to_string;
use std::num::ParseIntError;

// TODO: add tests and error handling to this package
pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
    .expect("something went wrong trying to read input file")
    .lines()
    .map(String::from)
    .collect()
}

pub fn from_input_line_to_i32_tuple(line: &str) -> Result<(i32,i32), ParseIntError> {
    let parts: Vec<&str>= line.split_whitespace().collect();

    let number_left = parts[0].parse::<i32>()?;
    let number_right = parts[1].parse::<i32>()?;

    return Ok((number_left, number_right))
}

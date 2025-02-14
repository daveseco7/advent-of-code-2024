use core::fmt;
use std::error::Error;
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

pub fn from_input_line_to_i32s(line: &str, expected_line_width: usize) -> Result<Vec<i32>, InputError> {
    let line_args = line.split_whitespace()
    .into_iter()
    .map(|n| match n.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(InputError::ParseError(e))
    })
    .collect::<Result<Vec<i32>, InputError>>();


    match line_args {
        Ok(line_args) => {
           // validate the expected number of arguments per line
            if line_args.len() != expected_line_width {
                return Err(InputError::UnexpectedLineColumnWidth)
            } 
            Ok(line_args)
        }
        Err(e) => Err(e),
    }
}


// Custom error to be returned when something went wrong while parsing 
// the input to something the downstream program can execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputError {
    ParseError(ParseIntError),
    UnexpectedLineColumnWidth,
}

impl Error for InputError {}

// Simple display func to help whenever we need to print error information
impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::ParseError(e) =>
                write!(f, "invalid i32s provided as str, err: {}",e),
            InputError::UnexpectedLineColumnWidth =>
                write!(f,"invalid number of columns provided"),
        }
    }
}


#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn tst_from_input_line_to_i32s_ok() {
        let result = from_input_line_to_i32s("1 2 3 43", 4); 
        let result = result.unwrap();

        assert_eq!(result, Vec::from([1,2,3,43]));
    }

    #[test]
    fn tst_from_input_line_to_i32s_parse_error() {
        let err = from_input_line_to_i32s("1 t 3 43", 4).unwrap_err();
        match err {
            InputError::ParseError(_) => {},
            e => panic!("expected parse error, got err: {}", e),
        }
    }

    #[test]
    fn tst_from_input_line_to_i32s_unexpected_line_column_width() {
        let err = from_input_line_to_i32s("1 2 3 43", 2).unwrap_err(); 
        assert_eq!(err, InputError::UnexpectedLineColumnWidth);
    }
}
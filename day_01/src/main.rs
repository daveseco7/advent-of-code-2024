use std::collections::HashMap;
use utils::{read_lines, from_input_line_to_i32s};


fn main() {
    println!("Please run tests to get the resuls!");
}

fn exe1(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut numbers_left: Vec<i32> = Vec::new();
    let mut numbers_right: Vec<i32> = Vec::new();

    // read and parse input into i32 tuple
    for line in lines {
        let result = from_input_line_to_i32s(&line,2);
        
        match result {
            Ok(v) => {
                numbers_left.push(v[0]);
                numbers_right.push(v[1]);
            }
            Err(e) => print!("err: {}",e)
        }
    }

    numbers_left.sort();
    numbers_right.sort();


    let mut distance_sum = 0;
    for i in 0..numbers_left.len() {
        let distance = numbers_left[i] - numbers_right[i];
        
        distance_sum += distance.abs();

    }

    distance_sum
}

fn exe2(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut numbers_left: Vec<i32> = Vec::new();
    let mut numbers_right:Vec <i32> = Vec::new();

    // read and parse input into i32 tuple
    for line in lines {
        let result = from_input_line_to_i32s(&line,2);
        
        match result {
            Ok(v) => {
                numbers_left.push(v[0]);
                numbers_right.push(v[1]);
            }
            Err(e) => print!("err: {}",e)
        }
    }

    // create frequency map for the right numbers
    let mut frequency_map_right: HashMap<i32, i32> = HashMap::new();
    for num in numbers_right {
        *frequency_map_right.entry(num).or_default() += 1;
    }

    let mut distance_sum = 0;
    
    for left_number in numbers_left.into_iter() {
        let frequency_value = match frequency_map_right.get(&left_number){
            Some(v) => &v,
            None => &0,
        };

        distance_sum += left_number * frequency_value
    }

    distance_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exe1_example() {
        assert_eq!(exe1("inputs/exe1_example.txt"), 11)
    }

    #[test]
    fn test_exe1_input() {
        assert_eq!(exe1("inputs/exe1_input.txt"), 765748)
    }

    #[test]
    fn test_exe2_example() {
        assert_eq!(exe2("inputs/exe2_example.txt"), 31)
    }

    #[test]
    fn test_exe2_input() {
        assert_eq!(exe2("inputs/exe2_input.txt"), 27732508)
    }
}
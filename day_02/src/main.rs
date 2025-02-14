use utils::{read_lines, from_input_line_to_i32_tuple};

fn main() {
    println!("Hello, world!");
}

fn exe1(filename: &str) -> i32 {
    let lines = read_lines(filename);

    return 1
}

struct Report {
    levels: Vec<i32>
}

fn fromStrToReport(line: &str) -> Result<Report> {
    let parts: Vec<&str>= line.split_whitespace().collect();

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
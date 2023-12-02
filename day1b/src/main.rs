use std::fs;
const INPUT_PATH: &str = "day1.input";

fn first_digit<I>(line: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    for c in line {
        if c.is_ascii_digit() {
            return c.to_digit(10);
        }
    }
    None
}

fn main() {
    let input_lines = fs::read_to_string(INPUT_PATH).unwrap_or(String::from(""));
    let mut calibration_sum: u32 = 0;
    for line in input_lines.lines() {
        let reverse_line = line.chars().rev();
        calibration_sum += 10 * first_digit(line.chars()).unwrap_or(0);
        calibration_sum += first_digit(reverse_line).unwrap_or(0);
    }

    println!("Calibration values sum to {calibration_sum}");
}

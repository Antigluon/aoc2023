use std::{collections::HashMap, fs};
const INPUT_PATH: &str = "day1.input";

fn spelled_out_digits(line: &str) -> Option<((usize, u32), (usize, u32))> {
    let digit_words: &HashMap<&str, u32> = &HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut digit_positions: HashMap<usize, u32> = HashMap::new();
    for digit in digit_words.keys() {
        for match_pos in line.match_indices(digit) {
            digit_positions.insert(match_pos.0, *digit_words.get(digit).unwrap());
        }
    }
    let max_pos = digit_positions.keys().max();
    match max_pos {
        Some(max_pos) => {
            let min_pos = digit_positions.keys().min().unwrap();
            let max_pos_digit = digit_positions.get(max_pos).unwrap();
            let min_pos_digit = digit_positions.get(min_pos).unwrap();
            Some(((*min_pos, *min_pos_digit), (*max_pos, *max_pos_digit)))
        }
        None => None,
    }
}

fn first_last_digit(line: &str) -> Option<(u32, u32)> {
    let mut first_digit: Option<u32> = None;
    let mut first_digit_pos: Option<usize> = None;
    let mut last_digit: Option<u32> = None;
    let mut last_digit_pos: Option<usize> = None;
    for (pos, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            first_digit = c.to_digit(10);
            first_digit_pos = Some(pos);
            break;
        }
    }
    for (pos, c) in line.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            last_digit = c.to_digit(10);
            last_digit_pos = Some(line.len() - (pos + 1));
            break;
        }
    }
    if let Some(first_digit) = first_digit {
        eprintln!(
            "first numeral: {first_digit} @ {pos}",
            pos = first_digit_pos.unwrap()
        );
    }
    if let Some(last_digit) = last_digit {
        eprintln!(
            "last numeral: {last_digit} @ {pos}",
            pos = last_digit_pos.unwrap()
        );
    }
    if let Some(((sp_first, sfirst), (sp_last, slast))) = spelled_out_digits(line) {
        eprintln!("spelled out first: {sfirst} @ {sp_first}");
        eprintln!("spelled out last: {slast} @ {sp_last}");
    }
    if let Some((
        (first_digitword_pos, first_spelled_digit),
        (last_digitword_pos, last_spelled_digit),
    )) = spelled_out_digits(line)
    {
        match first_digit {
            Some(_) => {
                eprintln!(
                    "first digit positions: {first_digit_pos} vs. {first_digitword_pos}",
                    first_digit_pos = first_digit_pos.unwrap(),
                    first_digitword_pos = first_digitword_pos,
                );
                if first_digitword_pos < first_digit_pos.unwrap() {
                    first_digit = Some(first_spelled_digit);
                }
                if last_digitword_pos > last_digit_pos.unwrap() {
                    last_digit = Some(last_spelled_digit);
                }
            }
            None => {
                first_digit = Some(first_spelled_digit);
                last_digit = Some(last_spelled_digit);
            }
        }
    }
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}

fn main() {
    let input_lines = fs::read_to_string(INPUT_PATH).unwrap_or(String::from(""));
    let mut calibration_sum: u32 = 0;
    for line in input_lines.lines() {
        if let Some((first_digit, last_digit)) = first_last_digit(line) {
            calibration_sum += (10 * first_digit) + last_digit;
            eprintln!(
                "Calibration Number: {calibration_number}",
                calibration_number = (10 * first_digit) + last_digit
            )
        }
    }

    println!("Calibration values sum to {calibration_sum}");
}

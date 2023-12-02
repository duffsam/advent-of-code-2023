use std::fs::read_to_string;

pub fn extract_digits(str_input: &str) -> Vec<u32> {
    str_input
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

pub fn find_digit(line: &str, reverse: bool) -> Option<u32> {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5",
        "6", "7", "8", "9",
    ];
    let mut min_position = None;
    let mut min_digit = None;

    for digit in &digits {
        if reverse {
            if let Some(position) = line.rfind(digit) {
                if position >= min_position.unwrap_or(position) {
                    min_position = Some(position);
                    min_digit = Some(digit);
                }
            }
        } else if let Some(position) = line.find(digit) {
            if position <= min_position.unwrap_or(position) {
                min_position = Some(position);
                min_digit = Some(digit);
            }
        }
    }

    min_digit.map(|digit| match *digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    })
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

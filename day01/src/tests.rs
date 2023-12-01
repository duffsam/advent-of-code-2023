#![cfg(test)]

use crate::lib::{day_01_part_1, day_01_part_2};

#[test]
pub fn day_01_part_1_test() {

    let mut test_input : Vec<String> = vec!();
    test_input.push("1abc2".to_owned());
    test_input.push("pqr3stu8vwx".to_owned());
    test_input.push("a1b2c3d4e5f".to_owned());
    test_input.push("treb7uchet".to_owned());
    let answer = 142;

    let total = day_01_part_1(&test_input);

    assert_eq!(answer, total);    
}

#[test]
pub fn day_01_part_2_test() {

    let mut test_input : Vec<String> = vec!();
    test_input.push("two1nine".to_owned());
    test_input.push("eightwothree".to_owned());
    test_input.push("abcone2threexyz".to_owned());
    test_input.push("xtwone3four".to_owned());
    test_input.push("4nineeightseven2".to_owned());
    test_input.push("zoneight234".to_owned());
    test_input.push("7pqrstsixteen".to_owned());
    let answer = 281;
    let total = day_01_part_2(&test_input);

    assert_eq!(answer, total);    
}
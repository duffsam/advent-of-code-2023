#![cfg(test)]

use helpers::read_lines;
use crate::parts::{day02_part_1, day02_part_2};

#[test]
pub fn day_02_part_1_test() {
    let test_input = read_lines("day02/inputs/test_input.txt");

    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    let answer = 8;

    let sum_game_number = day02_part_1(&test_input, max_red, max_blue, max_green);

    assert_eq!(answer, sum_game_number);
    
}

#[test]
pub fn day_02_part_2_test() {
    let test_input = read_lines("day02/inputs/test_input.txt");
    let answer = 2286;

    let total_power = day02_part_2(&test_input);

    assert_eq!(answer, total_power);
    
}
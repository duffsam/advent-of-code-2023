mod parts;
mod tests;
use helpers::read_lines;

fn main() {
    let test_input = read_lines("day02/inputs/input.txt");

    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    let sum_game_number = parts::day02_part_1(&test_input, max_red, max_blue, max_green);
    println!("Sum of the IDs of possible games = {}", sum_game_number);

    let total_power = parts::day02_part_2(&test_input);
    println!("Total power of these games is = {}", total_power);
}

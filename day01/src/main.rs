mod lib;
mod tests;
use helpers::read_lines;

fn main() {
    println!("Hello, world!");
    let lines = read_lines("day01/inputs/input.txt");
    let total = lib::day_01_part_1(&lines);
    println!("Part 1 total is {}", total);
    let total = lib::day_01_part_2(&lines);
    println!("Part 2 total is {}", total);        

}

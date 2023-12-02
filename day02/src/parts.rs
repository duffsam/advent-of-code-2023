use helpers::{parse_game, Round};


pub fn day02_part_1(input :&Vec<String>, max_red :u32, max_blue :u32, max_green :u32) -> u32 {
    let mut sum_game_number = 0;
    for line in input {
        let game = parse_game(&line);
        let mut condition_failed = false;
        for round in game.rounds {
            if round.green > max_green || round.red > max_red || round.blue > max_blue {
                condition_failed = true;
            }
        }
        if condition_failed == false {
            sum_game_number += game.number;
        }
    }
    return sum_game_number;
}

pub fn day02_part_2(input :&Vec<String>) -> u32{
    let mut total_power = 0;
    for line in input {
        let game = parse_game(&line);
        let mut fewest_content = Round{
            blue: 0,
            green: 0,
            red: 0,
        };
        for round in game.rounds {
            if round.green > fewest_content.green {
                fewest_content.green = round.green
            }
            if round.blue > fewest_content.blue {
                fewest_content.blue = round.blue
            }
            if round.red > fewest_content.red {
                fewest_content.red = round.red
            }
        }
        let game_power = fewest_content.red * fewest_content.blue * fewest_content.green;
        total_power += game_power;
    } 
    return total_power;
}
#[derive(Debug)]
pub struct Game {
    pub number :u32,
    pub rounds : Vec<Round>
}

#[derive(Debug)]
pub struct Round{
    pub blue : u32,
    pub green : u32,
    pub red : u32
}

pub fn parse_game(game_input :&str) -> Game {

    let game = find_game(&game_input);
    let rounds = extract_rounds(&game_input);
    Game {
        number: game.unwrap(),
        rounds: rounds.unwrap()
    }
}


pub fn find_game(line :&str) -> Result<u32, String>{
    let seperated: std::str::Split<'_, &str> = line.split(" ");
    for s in seperated {
        if s.contains(":") {
            let mut owned_str = s.to_owned();
            owned_str.pop().unwrap();
            let game = owned_str.trim().parse::<u32>().unwrap();
            return Ok(game);
        }
    }
    return Err(String::from("Not found. Make sure ':' is right after the integer"));
}

pub fn extract_rounds(line :&str) -> Result<Vec<Round>, String> {
    let seperated_colon = line.split(":").collect::<Vec<&str>>();
    let rounds_in_text = seperated_colon[1].split(";").collect::<Vec<&str>>();
    let mut rounds :Vec<Round> = vec!();
    for round_text in rounds_in_text {
        let mut ru  = Round {
            blue: 0,
            green: 0,
            red: 0,
        };
        let balls_in_text = round_text.split(",").collect::<Vec<&str>>();
        for ball_text in balls_in_text {
            let str_integers: String = ball_text.chars().filter(|c| c.is_digit(10)).collect();
            // Get the count
            let count = atoi::atoi::<u32>(str_integers.as_bytes());
            if count.is_some(){
                //Get the colour
                if ball_text.contains("green"){
                    ru.green = count.unwrap();
                } else if ball_text.contains("blue") {
                    ru.blue = count.unwrap();                
                } else if ball_text.contains("red") {
                    ru.red = count.unwrap();
                }
            } else {
                return Err(format!("Could not find any integers in {}", ball_text));
            }
        }
        rounds.push(ru);        
    }
    return Ok(rounds);
}

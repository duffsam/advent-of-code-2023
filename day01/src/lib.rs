use helpers::extract_digits;
use helpers::find_digit;

pub fn day_01_part_1(input : &Vec<String>) -> u32 {
    let mut total : u32 = 0;
    for l in input {
        let digits = extract_digits(&l);
        let first = match digits.first() {
            Some(val) => *val,
            None => 0,
        };
        let last = match digits.last() {
            Some(val) => *val,
            None => 0,
        };    
        total = total + (first*10) + last;
    };
    return total;
}

pub fn day_01_part_2(input : &Vec<String>) -> u32 {
    let mut total : u32 = 0;
    for line in input{
        let mut first = 0;
        let mut last = 0;

        first = match find_digit(&line, false) {
            Some(val) => val,
            None => 0,
        };
        last = match find_digit(&line, true) {
            Some(val) => val,
            None => 0,
        };
        total = total + (first*10) + last;        
    }
    return total;
}
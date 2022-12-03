use std::error::Error;
use std::fs;

// X - Rock 88
// Y - paper 89
// Z - scissors 90

// A - rock 65
// B - paper 66
// C - Scisscors 67

// draw - 3
// lose - 0
// win  - 6
fn calculate_score(x: u32, y: u32) -> u32 {
    // draw
    if y - x == 23 {
        3 + (y % 87)
    }
    // lost
    else if (y - x) == 22 || (y - x) == 25 {
        y % 87
    }
    // rest of cases - win
    else {
        6 + (y % 87)
    }
    // add 1 for rock, 2 for paper, 3 for scissors
}

fn part_1(data: &String) -> Result<u32, Box<dyn Error>> {
    let result = data
        .lines()
        .map(|strategy| {
            strategy
                .split_whitespace()
                .map(|letter| letter.parse::<char>().unwrap() as u32) // todo other way??
                .reduce(|rival, player| calculate_score(rival, player)) // could be done differently
                .unwrap_or_else(|| 0)
        })
        .sum::<u32>();
    Ok(result)
}

// X - Rock 88 -- lose
// Y - paper 89 -- draw
// Z - scissors 90 -- win

// A - rock 65
// B - paper 66
// C - Scisscors 67

// draw - 3
// lose - 0
// win  - 6
fn calculate_result(x: u32, y: u32) -> u32 {
    match y {
        // lost
        88 => {
            if x == 65 {
                return 3;
            } else {
                return (x % 64) - 1;
            }
        }
        // draw
        89 => {
            return 3 + (x % 64);
        }
        // win
        90 => {
            if x == 67 {
                return 7;
            } else {
                return 7 + (x % 64);
            }
        }
        _ => 0,
    }
}

fn part_2(data: &String) -> Result<u32, Box<dyn Error>> {
    let result = data
        .lines()
        .map(|strategy| {
            strategy
                .split_whitespace()
                .map(|letter| letter.parse::<char>().unwrap() as u32) // todo other way??
                .reduce(|rival, player| calculate_result(rival, player)) // could be done differently
                .unwrap_or_else(|| 0)
        })
        .sum::<u32>();
    Ok(result)
}

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./res/day02")?;
    if let Ok(answer) = part_1(&data) {
        println!("Day 2 part 1 answer: {}", answer);
    }
    if let Ok(answer) = part_2(&data) {
        println!("Day 2 part 2 answer: {}", answer);
    }
    Ok(())
}

#[cfg(test)]
mod day02 {
    use crate::*;

    #[test]
    fn part_1_test() {
        let data = fs::read_to_string("./res/day02").unwrap();
        assert!(part_1(&data).is_ok());
        assert_eq!(part_1(&data).unwrap(), 13809);
    }
    #[test]
    fn part_2_test() {
        let data = fs::read_to_string("./res/day02").unwrap();
        assert!(part_2(&data).is_ok());
        assert_eq!(part_2(&data).unwrap(), 12316);
    }
}

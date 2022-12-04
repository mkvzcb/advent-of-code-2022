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
fn calculate_score(moves: &[u8]) -> u8 {
    if moves[2] - moves[0] == 23 {
        return 3 + (moves[2] % 87);
    }
    // lost
    else if (moves[2] - moves[0]) == 22 || (moves[2] - moves[0]) == 25 {
        return moves[2] % 87;
    }
    // rest of cases - win
    else {
        return 6 + (moves[2] % 87);
    }
    // add 1 for rock, 2 for paper, 3 for scissors
}

fn part_1(data: &String) -> Result<u32, Box<dyn Error>> {
    let result = data.lines().fold(0, |acc, strategy| {
        acc + calculate_score(strategy.as_bytes()) as u32
    });
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
fn calculate_result(moves: &[u8]) -> u8 {
    match moves[2] {
        // lost
        88 => {
            if moves[0] == 65 {
                return 3;
            } else {
                return (moves[0] % 64) - 1;
            }
        }
        // draw
        89 => {
            return 3 + (moves[0] % 64);
        }
        // win
        90 => {
            if moves[0] == 67 {
                return 7;
            } else {
                return 7 + (moves[0] % 64);
            }
        }
        _ => 0,
    }
}

fn part_2(data: &String) -> Result<u32, Box<dyn Error>> {
    let result = data.lines().fold(0, |acc, strategy| {
        acc + calculate_result(strategy.as_bytes()) as u32
    });
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

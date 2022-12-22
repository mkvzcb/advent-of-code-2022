// X - Rock 88
// Y - paper 89
// Z - scissors 90

// A - rock 65
// B - paper 66
// C - Scisscors 67

// draw - 3
// lose - 0
// win  - 6
fn calculate_score_part_1(moves: &[u8]) -> u8 {
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

// X -- lose
// Y -- draw
// Z -- win

fn calculate_score_part_2(moves: &[u8]) -> u8 {
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

//

pub fn part_2(data: &String) -> u32 {
    let result = data.lines().fold(0, |acc, strategy| {
        acc + calculate_score_part_2(strategy.as_bytes()) as u32
    });

    result
}

pub fn part_1(data: &String) -> u32 {
    let result = data.lines().fold(0, |acc, strategy| {
        acc + calculate_score_part_1(strategy.as_bytes()) as u32
    });

    result
}
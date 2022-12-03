use std::error::Error;
use std::fs;

fn part_1(data: &String) -> Result<i32, Box<dyn Error>> {
    let result = data
        .split("\n\n")
        .map(|bag| {
            bag.split("\n")
                .map(|calorie| calorie.parse::<i32>().unwrap_or_else(|_| 0))
                .sum()
        })
        .reduce(|x, y| if y > x { y } else { x });
    Ok(result.unwrap())
}

fn part_2(data: &String) -> Result<i32, Box<dyn Error>> {
    let mut result = data
        .split("\n\n")
        .map(|bag| {
            bag.split("\n")
                .map(|calorie| calorie.parse::<i32>().unwrap_or_else(|_| 0))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    result.sort_by(|x, y| y.cmp(x));
    Ok(result[0] + result[1] + result[2])
}

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./res/day01").unwrap();

    if let Ok(answer) = part_1(&data) {
        println!("Day 1 part 1 answer: {}", answer);
    }
    if let Ok(answer) = part_2(&data) {
        println!("Day 1 part 2 answer: {}", answer);
    }
    Ok(())
}

#[cfg(test)]
mod day01 {
    use crate::*;

    #[test]
    fn part_1_test() {
        let data = fs::read_to_string("./res/day01").unwrap();
        assert!(part_1(&data).is_ok());
        assert_eq!(part_1(&data).unwrap(), 74394);
    }

    #[test]
    fn part_2_test() {
        let data = fs::read_to_string("./res/day01").unwrap();

        assert!(part_2(&data).is_ok());
        assert_eq!(part_2(&data).unwrap(), 212836);
    }
}

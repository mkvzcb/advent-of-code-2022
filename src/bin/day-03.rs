use std::error::Error;
use std::fs;

fn piority(number: u32) -> u32 {
    if (number) < 91 {
        number - 38
    } else {
        number - 96
    }
}

pub fn part_1(data: &String) -> Result<u32, Box<dyn Error>> {
    let result = data
        .lines()
        .map(|x| {
            let (first, second) = x.split_at(x.len() / 2);
            piority(first
                .chars()
                .filter(|k| second.contains(*k)).next().unwrap() as u32)

        })
        .sum();
    Ok(result)
}

fn part_2(data: &String) -> Result<u32, Box<dyn Error>> {
    todo!()
}

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./res/day03")?;
    if let Ok(answer) = part_1(&data) {
        println!("Day 3 part 1 answer: {}", answer);
    }
    Ok(())
}

#[cfg(test)]
mod day03 {
    use crate::*;

    #[test]
    fn part_1_test() {
        let data = fs::read_to_string("./res/day03").unwrap();
        assert!(part_1(&data).is_ok());
        assert_eq!(part_1(&data).unwrap(), 8298);
    }

}

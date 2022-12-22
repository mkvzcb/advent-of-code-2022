use itertools::Chunk;
use itertools::Itertools;

fn piority(number: u32) -> u32 {
    if (number) < 91 {
        number - 38
    } else {
        number - 96
    }
}

fn process_line(line: &str) -> u32 {
    let (first, second) = line.split_at(line.len() / 2);
    piority(
        first
            .chars()
            .filter(|k| second.contains(*k))
            .next()
            .unwrap() as u32,
    )
}

fn process_multiple_lines(chunk: Chunk<impl Iterator<Item = String>>) -> u32 {
    chunk
        .reduce(|x, y| x.chars().filter(|k| y.contains(*k)).collect())
        .unwrap()
        .as_bytes()[0] as u32
}

// could be a lot shorter
pub fn part_2(data: &String) -> u32 {
    let result = data
        .lines()
        .map(|x| String::from(x))
        .chunks(3)
        .into_iter()
        .fold(0, |acc, chunk| acc + piority(process_multiple_lines(chunk)));
        
    result
}

pub fn part_1(data: &String) -> u32 {
    let result = data.lines().map(|line| process_line(line)).sum();

    result
}

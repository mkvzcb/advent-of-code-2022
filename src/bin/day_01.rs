use advent_of_code_2022::problems::day_01;

fn main() -> std::io::Result<()> {
    if let Ok(data) = std::fs::read_to_string("./res/day01") {
        println!("Day 1 part 1 answer: {}", day_01::part_1(&data));

        println!("Day 1 part 2 answer: {}", day_01::part_2(&data));
    } else {
        eprintln!("Unable to read the file")
    }
    Ok(())
}

use advent_of_code_2022::problems::day_03;

fn main() -> std::io::Result<()> {
    if let Ok(data) = std::fs::read_to_string("./res/day03") {
        println!("Day 3 part 1 answer: {}", day_03::part_1(&data));
        
        println!("Day 3 part 2 answer: {}", day_03::part_2(&data));
    } else {
        eprintln!("Unable to read the file")
    }
    Ok(())
}

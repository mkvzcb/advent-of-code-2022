use advent_of_code_2022::problems::day_02;

fn main() -> std::io::Result<()> {
    if let Ok(data) = std::fs::read_to_string("./res/day02") {
        println!("Day 2 part 1 answer: {}", day_02::part_1(&data));
        
        println!("Day 2 part 2 answer: {}", day_02::part_2(&data));
    } else {
        eprintln!("Unable to read the file")
    }
    Ok(())
}

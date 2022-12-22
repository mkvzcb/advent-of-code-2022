use advent_of_code_2022::problems::day_04;

fn main() -> std::io::Result<()> {
    if let Ok(data) = std::fs::read_to_string("./res/day04") {
        println!("Day 4 part 1 answer: {}", day_04::part_1(&data));
        
        println!("Day 4 part 2 answer: {}", day_04::part_2(&data));
    } else {
        eprintln!("Unable to read the file")
    }
    Ok(())
}

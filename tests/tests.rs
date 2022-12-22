mod consts;

use consts::*;
use advent_of_code_2022::problems::*;
// DAY 01

#[test]
fn day_01_part_1() {
    assert_eq!(day_01::part_1(&String::from(DAY01)), 56278);
}

#[test]
fn day_01_part_2() {
    assert_eq!(day_01::part_2(&String::from(DAY01)), 167193);
}

// DAY 02

#[test]
fn day_02_part_1() {
    assert_eq!(day_02::part_1(&String::from(DAY02)), 227);
}
#[test]
fn day_02_part_2() {
    assert_eq!(day_02::part_2(&String::from(DAY02)), 188);
}

// DAY 03

#[test]
fn day_03_part_1() {
    assert_eq!(day_03::part_1(&String::from(DAY03)), 1344);
}
#[test]
fn day_03_part_2() {
    assert_eq!(day_03::part_2(&String::from(DAY03)), 475);
}


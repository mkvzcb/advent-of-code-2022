pub fn part_2(data: &String) -> i32 {
    let mut result = data
        .split("\n\n")
        .map(|bag| {
            bag.split("\n").fold(0, |acc, calorie| {
                acc + i32::from_str_radix(calorie, 10).unwrap_or(0)
            })
        })
        .collect::<Vec<i32>>();

    result.sort_by(|x, y| y.cmp(x));

    result[0] + result[1] + result[2]
}

pub fn part_1(data: &String) -> i32 {
    let result = data
        .split("\n\n")
        .map(|bag| {
            bag.split("\n").fold(0, |acc, calorie| {
                acc + i32::from_str_radix(calorie, 10).unwrap_or(0)
            })
        })
        .reduce(|x, y| if y > x { y } else { x });

    result.unwrap()
}

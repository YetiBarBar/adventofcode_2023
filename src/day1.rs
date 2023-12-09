fn main() {
    let data: Vec<_> = include_str!("../data/day_2023_1.data").lines().collect();

    println!("Part 1: {}", part1(data.iter()));
    println!("Part 2: {}", part2(&data));
}

fn part1<T: AsRef<str>>(data: impl Iterator<Item = T>) -> usize {
    data.map(|line| {
        let digits: Vec<char> = line.as_ref().chars().filter(char::is_ascii_digit).collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<usize>()
            .unwrap()
    })
    .sum()
}

fn part2(data: &[&str]) -> usize {
    part1(data.iter().map(|line| transform_str(line)))
}

fn transform_str(line: &str) -> String {
    [
        ("twone", "21"),
        ("oneight", "18"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .fold(line.to_string(), |acc, (pattern, newstr)| {
        acc.replace(pattern, newstr)
    })
}

fn main() {
    let data = include_str!("../data/day_2023_1.data")
        .lines()
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &[&str]) -> usize {
    data.iter()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part2(data: &[&str]) -> usize {
    let data = data
        .iter()
        .map(|line| {
            line.replace("twone", "21")
                .replace("oneight", "18")
                .replace("threeight", "38")
                .replace("fiveight", "58")
                .replace("sevenine", "79")
                .replace("eightwo", "82")
                .replace("eighthree", "83")
                .replace("nineight", "98")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
        })
        .collect::<Vec<_>>();
    let ref_data: Vec<&str> = data.iter().map(std::convert::AsRef::as_ref).collect();
    part1(&ref_data)
}

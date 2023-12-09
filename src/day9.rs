fn main() {
    let data: Vec<Vec<i64>> = include_str!("../data/day_2023_9.data")
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    println!("Part 1 : {}", part1(&data));
    println!("Part 2 : {}", part2(&data));
}

fn part1(data: &[Vec<i64>]) -> i64 {
    data.iter()
        .map(|v| v.last().copied().unwrap() + compute_next_part1(v))
        .sum()
}

fn part2(data: &[Vec<i64>]) -> i64 {
    let data: Vec<Vec<i64>> = data
        .iter()
        .map(|v| v.iter().rev().copied().collect())
        .collect();
    part1(&data)
}

fn compute_next_part1(data: &[i64]) -> i64 {
    let new_line: Vec<i64> = data
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    if new_line.iter().all(|v| v == &0) {
        0
    } else {
        compute_next_part1(&new_line) + new_line.last().copied().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        let test_data: Vec<Vec<i64>> = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        assert_eq!(part1(&test_data), 114);
    }

    #[test]
    fn test_day9_part2() {
        let test_data: Vec<Vec<i64>> = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        assert_eq!(part2(&test_data), 2);
    }
}

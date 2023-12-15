fn main() {
    let data: Vec<(&str, Vec<usize>)> = include_str!("../data/day_2023_12.data")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(l, val)| {
            (
                l,
                val.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    let part1: u128 = data
        .iter()
        .map(|(line, v)| brute_force_combinations(line, v))
        .sum();

    println!("Part 1: {part1}");

    /*  let data_part2: Vec<(String, Vec<usize>)> = data
    .iter()
    .map(|(line, values)| {
        let new_str = format!("{line}?{line}?{line}?{line}?{line}");
        let mut v = vec![];
        for _ in 0..5 {
            v.extend(values.iter().copied())
        }
        (new_str, v)
    })
    .collect(); */
    // let part2: u128 = data_part2
    //     .iter()
    //     .map(|(line, v)| brute_force_combinations(line, v))
    //     .sum();
    //
    // println!("Part 2: {}", part2);
}

fn evaluate_string(line: &str, values: &Vec<usize>) -> bool {
    let substrings: Vec<&str> = line.split('.').collect();
    let computed_seq: Vec<usize> = substrings
        .iter()
        .map(|sub| sub.trim_matches('.').len())
        .filter(|l| l != &0)
        .collect();

    &computed_seq == values
}

fn brute_force_combinations(line: &str, values: &Vec<usize>) -> u128 {
    let count = line.chars().filter(|chr| chr == &'?').count();

    let range = 0..2_usize.pow(u32::try_from(count).unwrap());

    let mut res = 0;
    for idx in range {
        let mut new_str: Vec<char> = line.chars().collect();
        for idx2 in 0..count {
            let subst = new_str.iter_mut().find(|chr| chr == &&'?').unwrap();
            if idx & (1 << idx2) == 0 {
                *subst = '.';
            } else {
                *subst = '#';
            }
        }
        let new_str: String = new_str.iter().collect();
        if evaluate_string(&new_str, values) {
            res += 1;
        }
    }
    println!("{res}");
    res
}

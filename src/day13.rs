fn main() {
    let data: Vec<Vec<Vec<char>>> = include_str!("../data/day_2023_13.data")
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.chars().collect()).collect())
        .collect();

    println!(
        "Part 1: {}",
        data.iter()
            .map(|v| find_symetries(v, false).into_iter().next())
            .map(Option::unwrap)
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        data.iter()
            .map(find_smudge)
            .map(Option::unwrap)
            .sum::<usize>()
    );
}

fn find_symetries(bloc: &Vec<Vec<char>>, is_transposed: bool) -> Vec<usize> {
    let mut ret = vec![];
    for idx in 1..(bloc.len()) {
        if bloc[0..idx]
            .iter()
            .rev()
            .zip(bloc[idx..].iter())
            .all(|(a, b)| a == b)
        {
            ret.push(100 * idx);
        }
    }
    if !is_transposed {
        let mut transposed = vec![vec![]; bloc[0].len()];
        for line in bloc {
            for (pos, chr) in line.iter().enumerate() {
                transposed[pos].push(*chr);
            }
        }
        ret.extend(find_symetries(&transposed, true).iter().map(|v| v / 100));
    }
    ret
}

fn find_smudge(bloc: &Vec<Vec<char>>) -> Option<usize> {
    let (len_x, len_y) = (bloc[0].len(), bloc.len());
    let old = find_symetries(bloc, false).into_iter().next().unwrap();

    for y in 0..len_y {
        for x in 0..len_x {
            let mut cloned = bloc.clone();
            let mut row = cloned.get(y).unwrap().clone();

            row[x] = ((row[x] as u8) ^ b'.' ^ b'#') as char;

            cloned[y] = row;
            if let Some(value) = find_symetries(&cloned, false).iter().find(|v| v != &&old) {
                return Some(*value);
            }
        }
    }
    None
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn day_13_part1() {
        let data: Vec<Vec<Vec<char>>> = include_str!("../data/day_2023_13_test.data")
            .split("\n\n")
            .map(|s| s.lines().map(|s| s.chars().collect()).collect())
            .collect();
        assert_eq!(
            data.iter()
                .map(|v| find_symetries(v, false).into_iter().next().unwrap())
                .sum::<usize>(),
            405
        )
    }

    #[test]
    fn day_13_part2() {
        let data: Vec<Vec<Vec<char>>> = include_str!("../data/day_2023_13_test.data")
            .split("\n\n")
            .map(|s| s.lines().map(|s| s.chars().collect()).collect())
            .collect();
        assert_eq!(
            data.iter()
                .map(find_smudge)
                .map(Option::unwrap)
                .sum::<usize>(),
            400
        )
    }
}

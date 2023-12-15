fn main() {
    let data: Vec<&str> = include_str!("../data/day_2023_15.data")
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .collect();

    let part1: usize = data.iter().map(|data| elf_hash(data)).sum();
    println!("Part 1: {part1}");
    println!("Part 2: {}", part2(data));
}

fn elf_hash(text: &str) -> usize {
    text.as_bytes().iter().fold(0, |mut acc: usize, chr| {
        acc += *chr as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part2(data: Vec<&str>) -> usize {
    let mut boxes: Vec<DayBox> = (0..256).map(DayBox::new).collect();

    for line in data {
        if line.ends_with('-') {
            let name = line.trim_end_matches('-');
            let position = elf_hash(name);
            boxes[position].remove_item(name);
        } else {
            let (name, value) = line.split_once('=').unwrap();
            let value: usize = value.parse().unwrap();
            let position = elf_hash(name);
            boxes[position].insert_or_modify(name, value);
        }
    }

    boxes.iter().map(DayBox::evaluate).sum()
}

struct DayBox<'a> {
    index: usize,
    lenses: Vec<Lens<'a>>,
}

impl<'a> DayBox<'a> {
    fn evaluate(&self) -> usize {
        self.lenses
            .iter()
            .zip(1..)
            .map(|(lens, idx)| lens.power * (self.index + 1) * idx)
            .sum()
    }

    fn remove_item(&mut self, item: &str) {
        if let Some(index) = self.lenses.iter().position(|lens| lens.name == item) {
            self.lenses.remove(index);
        }
    }

    fn insert_or_modify(&mut self, item: &'a str, value: usize) {
        if let Some(index) = self.lenses.iter().position(|lens| lens.name == item) {
            let lens = self.lenses.get_mut(index).unwrap();
            lens.set_power(value);
        } else {
            self.lenses.push(Lens::new(item, value));
        }
    }

    fn new(index: usize) -> Self {
        Self {
            index,
            lenses: vec![],
        }
    }
}

#[derive(Debug)]
struct Lens<'a> {
    name: &'a str,
    power: usize,
}

impl<'a> Lens<'a> {
    fn new(name: &'a str, power: usize) -> Self {
        Self { name, power }
    }

    fn set_power(&mut self, power: usize) {
        self.power = power;
    }
}

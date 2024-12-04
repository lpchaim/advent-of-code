use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let (lhs, rhs): (Vec<_>, Vec<_>) = contents
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|it| it.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1])
        })
        .unzip();
    part1(&lhs, &rhs);
    part2(&lhs, &rhs);
}

fn part1(lhs: &[u32], rhs: &[u32]) {
    let mut lhs = lhs.to_owned();
    lhs.sort_unstable();
    let lhs = lhs.into_iter().rev();

    let mut rhs = rhs.to_owned();
    rhs.sort_unstable();
    let rhs = rhs.into_iter().rev();

    let sum: u32 = lhs.zip(rhs).map(|(l, r)| l.abs_diff(r)).sum();
    println!("{:#?}", sum);
}

fn part2(lhs: &[u32], rhs: &[u32]) {
    let mut ocurrences: HashMap<u32, u32> = HashMap::new();
    rhs.iter().for_each(|it| {
        if !ocurrences.contains_key(it) {
            ocurrences.insert(*it, 0);
        }
        let val = ocurrences.get_mut(it).unwrap();
        *val += 1;
    });
    let sum = lhs
        .iter()
        .map(|it| *it * ocurrences.get(it).unwrap_or(&0))
        .sum::<u32>();
    println!("{:#?}", sum);
}

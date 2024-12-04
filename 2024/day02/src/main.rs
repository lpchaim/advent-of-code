use std::fs;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<Vec<i32>> {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn part1() {
    solve(0)
}

fn part2() {
    solve(1)
}

fn solve(_bad_parts: u8) {
    let n_safe = parse()
        .into_iter()
        .filter_map(|line| line_is_valid(&line).then_some(line))
        .count();
    println!("{:#?}", n_safe);
}

fn line_is_valid(line: &[i32]) -> bool {
    let first_diff = line.get(1).unwrap() - line.get(0).unwrap();
    line.iter().enumerate().skip(1).all(|(i, curr)| {
        let prev = line[i - 1];
        let diff = curr - prev;
        diff.abs() >= 1
            && diff.abs() <= 3
            && match first_diff {
                _ if first_diff > 0 => diff.is_positive(),
                _ => diff.is_negative(),
            }
    })
}

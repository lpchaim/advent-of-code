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
    solve(false)
}

fn part2() {
    solve(true)
}

fn solve(permissive: bool) {
    let n_safe = parse()
        .into_iter()
        .filter_map(|line| {
            let mostly_ascending = line
                .windows(2)
                .filter(|arr| (arr[1] - arr[0]).is_positive())
                .count()
                >= (line.len() / 2);
            let valid_indices = get_valid_indices(&line, mostly_ascending);
            let mut invalid_indices = valid_indices
                .iter()
                .enumerate()
                .filter_map(|(i, valid)| (!*valid).then_some(i));
            let n_invalid = invalid_indices.clone().count();
            match n_invalid {
                0 => Some(line),
                _ if permissive => invalid_indices
                    .find(|index| {
                        let mut line = line.clone();
                        line.remove(*index);
                        get_valid_indices(&line, mostly_ascending)
                            .iter()
                            .all(|valid| *valid)
                    })
                    .and(Some(line))
                    .or(None),
                _ => None
            }
        })
        .count();
    println!("{:#?}", n_safe);
}

fn get_valid_indices(line: &[i32], ascending: bool) -> Vec<bool> {
    let mut valid = vec![true; line.len()];
    line.iter().enumerate().skip(1).for_each(|(i, curr)| {
        let prev = line[i - 1];
        let diff = curr - prev;
        let is_valid = diff.abs() >= 1
            && diff.abs() <= 3
            && ((ascending && diff.is_positive()) || (!ascending && diff.is_negative()));
        if !is_valid {
            valid[i - 1] = false;
            valid[i] = false;
        }
    });
    valid
}

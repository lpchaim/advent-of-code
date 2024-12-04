use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let mut enable = true;
    let sum: u32 = Regex::find_iter(
        &Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))")
            .expect("Unable to parse regex"),
        contents.as_str(),
    )
    .map(|m| match m.as_str() {
        r"do()" => {
            enable = true;
            0
        }
        r"don't()" => {
            enable = false;
            0
        }
        _ if !enable => 0,
        mul => Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .expect("Unable to parse regex")
            .captures(mul)
            .unwrap()
            .iter()
            .skip(1)
            .filter_map(|it| match it {
                Some(num) => Some(num.as_str().parse::<u32>().unwrap()),
                _ => None,
            })
            .reduce(|acc, it| acc * it)
            .unwrap_or(0),
    })
    .sum();
    println!("{:#?}", sum);
}

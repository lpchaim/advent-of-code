use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let sum = contents
        .split("mul(")
        .filter_map(|og| {
            let (lhs, remainder) = og.split_once(',').unwrap_or(("", ""));
            let (rhs, _) = remainder.split_once(')').unwrap_or(("", ""));
            let lhs = lhs.parse::<u32>();
            let rhs = rhs.parse::<u32>();
            match (lhs, rhs) {
                (Ok(x), Ok(y)) => Some(x * y),
                _ => None,
            }
        })
        .sum::<u32>();
    print!("{:#?}", sum);
}

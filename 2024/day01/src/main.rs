use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let (mut lhs, mut rhs): (Vec<_>, Vec<_>) = contents
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|it| it.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            (nums[0], nums[1])
        })
        .unzip();

    lhs.sort_unstable();
    let lhs = lhs.into_iter().rev();

    rhs.sort_unstable();
    let rhs = rhs.into_iter().rev();

    let sum: u16 = lhs.zip(rhs).map(|(l, r)| l.abs_diff(r)).sum();
    println!("{:#?}", sum);
}

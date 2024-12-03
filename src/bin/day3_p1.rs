use lazy_regex::regex;
use std::fs;

fn main() -> anyhow::Result<()> {
    let mul_regex = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
    let content = fs::read_to_string("inputs/day3/input.txt")?;
    let mut sum = 0;
    for (_, [first, second]) in mul_regex.captures_iter(&content).map(|c| c.extract()) {
        let first = first.parse::<i32>()?;
        let second = second.parse::<i32>()?;
        sum += first * second;
    }
    println!("Sum: {sum}");
    Ok(())
}

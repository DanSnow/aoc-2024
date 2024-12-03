use lazy_regex::regex;
use std::fs;

fn main() -> anyhow::Result<()> {
    let parse_regex = regex!(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))");
    let content = fs::read_to_string("inputs/day3/input.txt")?;
    let mut sum = 0;
    let mut enabled = true;
    for capture in parse_regex.captures_iter(&content) {
        match capture.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ if enabled == false => (),
            _ => {
                let first = capture.get(1).unwrap().as_str();
                let second = capture.get(2).unwrap().as_str();
                let first = first.parse::<i32>()?;
                let second = second.parse::<i32>()?;
                sum += first * second;
            }
        }
    }
    println!("Sum: {sum}");
    Ok(())
}

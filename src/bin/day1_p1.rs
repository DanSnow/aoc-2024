use std::fs;

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("inputs/day1/input.txt")?;
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in content.lines() {
        if !line.is_empty() {
            let mut iter = line.split_ascii_whitespace();
            list1.push(iter.next().unwrap().parse::<i32>().unwrap());
            list2.push(iter.last().unwrap().parse::<i32>().unwrap());
        }
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let mut sum = 0;
    for (l, r) in list1.iter().zip(list2.iter()) {
        sum += (l - r).abs()
    }
    println!("Sum: {}", sum);
    Ok(())
}

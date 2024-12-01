use std::{collections::HashMap, fs};

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("inputs/day1/input.txt")?;
    let mut list1 = vec![];
    let mut list2 = HashMap::<i32, i32>::new();
    for line in content.lines() {
        if !line.is_empty() {
            let mut iter = line.split_ascii_whitespace();
            list1.push(iter.next().unwrap().parse::<i32>().unwrap());
            let n2 = iter.last().unwrap().parse::<i32>().unwrap();
            list2
                .entry(n2)
                .and_modify(|c| {
                    *c += 1;
                })
                .or_insert(1);
        }
    }
    list1.sort_unstable();
    let mut sum = 0;
    for n in list1.iter().copied() {
        sum += n * list2.get(&n).unwrap_or(&0);
    }
    println!("Sum: {}", sum);
    Ok(())
}

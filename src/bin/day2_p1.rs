use std::fs;

fn is_safe(line: &str) -> bool {
    let mut iter = line.split_whitespace().map(|num| match num.parse::<i32>() {
        Ok(num) => num,
        Err(err) => panic!("invalid input {err:?}"),
    });
    let mut first_sign = 0;
    let mut prev_num = iter.next().unwrap();
    for num in iter {
        if first_sign == 0 {
            first_sign = (num - prev_num).signum();
            if first_sign == 0 {
                return false;
            }

            if !(1..=3).contains(&(num - prev_num).abs()) {
                return false;
            }
            prev_num = num;
            continue;
        }
        if (num - prev_num).signum() != first_sign || !(1..=3).contains(&(num - prev_num).abs()) {
            return false;
        }
        prev_num = num;
    }
    true
}

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("inputs/day2/input.txt")?;
    let mut safe_count = 0;
    for line in content.lines() {
        if is_safe(line) {
            safe_count += 1;
        }
    }
    println!("Safe: {safe_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert_eq!(false, is_safe("6 8 9 11 14 12"));
        assert_eq!(false, is_safe("31 33 36 39 42 42"));
        assert_eq!(false, is_safe("5 6 7 9 11 13 17"));
        assert_eq!(false, is_safe("4 8 9 11 12 15"));
        assert_eq!(true, is_safe("93 91 89 88 87 84"));
    }
}

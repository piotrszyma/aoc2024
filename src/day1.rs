use std::collections::HashMap;

use crate::file_utils::read_lines_from_file;

fn i64_pair_from_line(line: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut pairs = line.split_whitespace();
    let first: i64 = pairs.next().ok_or("first number not found in line")?.parse()?;
    let second: i64 = pairs.next().ok_or("second number not found in line")?.parse()?;
    Ok((first, second))
}

fn read_number_pairs(filepath: &str) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    read_lines_from_file(filepath)
        .map(|line| i64_pair_from_line(&line?))
        .collect()
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let pairs = read_number_pairs(input_path)?;

    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum())
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let pairs = read_number_pairs(input_path)?;
    let (left, right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    let counter: HashMap<_, _> = right.iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_default() += 1;
        acc
    });

    Ok(left
        .into_iter()
        .map(|val| val * counter.get(&val).unwrap_or(&0))
        .sum())
}

pub fn task1() -> Result<i64, Box<dyn std::error::Error>> {
    task1_run("data/day1.txt")
}

pub fn task2() -> Result<i64, Box<dyn std::error::Error>> {
    task2_run("data/day1.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1_test_data() {
        assert_eq!(11, task1_run("data/day1_test.txt").unwrap())
    }

    #[test]
    fn task1() {
        assert_eq!(2066446, task1_run("data/day1.txt").unwrap())
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(31, task2_run("data/day1_test.txt").unwrap())
    }

    #[test]
    fn task2() {
        assert_eq!(24931009, task2_run("data/day1.txt").unwrap())
    }
}

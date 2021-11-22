use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn i64_pair_from_line(line: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut pairs = line.split_whitespace();
    let first: i64 = pairs.next().ok_or("missing first number")?.parse()?;
    let second: i64 = pairs.next().ok_or("missing second number")?.parse()?;
    Ok((first, second))
}

fn read_number_pairs(path: &str) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| i64_pair_from_line(&line.unwrap()))
        .collect()
}

fn task1_run(input_path: &str) -> i64 {
    let pairs = read_number_pairs(input_path).unwrap();

    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn task2_run(input_path: &str) -> i64 {
    let pairs = read_number_pairs(input_path).unwrap();
    let (left, right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    let counter: HashMap<_, _> = right.iter()
        .fold(HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_default() += 1;
            acc
        });

    left.into_iter()
        .map(|val| val * counter.get(&val).unwrap_or(&0))
        .sum()
}

pub fn task1() -> i64 {
    task1_run("data/day1.txt")
}

pub fn task2() -> i64 {
    task2_run("data/day1.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1_test_data() {
        assert_eq!(11, task1_run("data/day1_test.txt"))
    }

    #[test]
    fn task1() {
        assert_eq!(2066446, task1_run("data/day1.txt"))
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(31, task2_run("data/day1_test.txt"))
    }

    #[test]
    fn task2() {
        assert_eq!(24931009, task2_run("data/day1.txt"))
    }
}

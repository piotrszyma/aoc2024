use std::{collections::HashSet, error::Error, fmt, hash::Hash};

use crate::file_utils::read_lines_from_file_v2;

fn is_eq_possible(result: &i64, ops: &[i64]) -> bool {
    if ops.len() == 1 {
        return *result == ops[0];
    }

    let op1 = ops[0];
    let op2 = ops[1];
    let rest = &ops[2..];

    let new_ops_variants = [op1 + op2, op1 * op2];

    new_ops_variants.iter().any(|&new_op| {
        let new_ops: Vec<i64> = std::iter::once(new_op)
            .chain(rest.iter().cloned())
            .collect();
        is_eq_possible_v2(result, &new_ops)
    })
}

fn concat_i64(a: i64, b: i64) -> i64 {
    let concatenated = format!("{}{}", a, b); // Concatenate as strings
    concatenated
        .parse::<i64>()
        .expect("Failed to parse concatenated number")
}

fn is_eq_possible_v2(result: &i64, ops: &[i64]) -> bool {
    if ops.len() == 1 {
        return *result == ops[0];
    }

    let op1 = ops[0];
    let op2 = ops[1];
    let rest = &ops[2..];

    let new_ops_variants = [op1 + op2, op1 * op2, concat_i64(op1, op2)];

    new_ops_variants.iter().any(|&new_op| {
        let new_ops: Vec<i64> = std::iter::once(new_op)
            .chain(rest.iter().cloned())
            .collect();
        is_eq_possible_v2(result, &new_ops)
    })
}

fn task_run(path: &str, checker: fn(&i64, &[i64]) -> bool) -> Result<i64, Box<dyn Error>> {
    let lines = read_lines_from_file_v2(path);

    let sum: i64 = lines
        .filter_map(|line| {
            let (expected_result, ops) = line.split_once(": ")?;

            let expected_result: i64 = expected_result.parse().ok()?;

            let ops: Vec<i64> = ops
                .split_whitespace()
                .map(|o| o.parse::<i64>().ok())
                .collect::<Option<Vec<_>>>()?;

            Some((expected_result, ops))
        })
        .filter(|(r, ops)| checker(r, ops))
        .map(|(r, _)| r)
        .sum();

    Ok(sum)
}

fn task1_run(path: &str) -> Result<i64, Box<dyn Error>> {
    task_run(path, is_eq_possible)
}

fn task2_run(path: &str) -> Result<i64, Box<dyn Error>> {
    task_run(path, is_eq_possible_v2)
}

pub fn task1() -> Result<i64, Box<dyn Error>> {
    task1_run("data/day7_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn Error>> {
    task2_run("data/day7_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1_test_data_test() {
        assert_eq!(3749, task1_run("data/day7_test.txt").unwrap())
    }

    #[test]
    fn task1_test() {
        assert_eq!(5067, task1_run("data/day7.txt").unwrap())
    }

    #[test]
    fn task2_test_data_test() {
        assert_eq!(11387, task2_run("data/day7_test.txt").unwrap())
    }

    #[test]
    fn task2_test() {
        assert_eq!(124060392153684, task2_run("data/day7.txt").unwrap())
    }
}

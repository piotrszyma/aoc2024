use crate::file_utils::read_lines_from_file;

fn is_monotonic(seq: &[i64]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..seq.len() {
        let prev_idx = i - 1;
        if seq[i] > seq[prev_idx] {
            decreasing = false;
        } else if seq[i] < seq[prev_idx] {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

// Slowly - diff between each neighbours must be between 1 & 3.
fn is_slowly_increasing(seq: &[i64]) -> bool {
    for i in 1..seq.len() {
        let prev = seq[i - 1];
        let next = seq[i];
        let diff = (prev - next).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_safe(seq: &[i64]) -> bool {
    is_monotonic(seq) & is_slowly_increasing(seq)
}

fn is_safe_v1(seq: &[i64]) -> bool {
    is_safe(seq)
}

fn is_safe_v2(seq: &[i64]) -> bool {
    if is_safe_v1(seq) {
        // If the original sequence is safe then we all good.
        return true;
    }

    // Naive approach where we create a copy of vector with each item removed.
    for i in 0..seq.len() {

        // TODO: create a wrapper over slice? with specific item hidden?
        let mut a_vec = seq.to_vec();
        a_vec.remove(i);

        if is_safe(a_vec.as_slice()) {
            return true;
        }
    }

    return false;
}

fn i64_vec_from_line(line: String) -> Vec<i64> {
    return line
        .split_whitespace()
        .map(|raw| raw.parse().unwrap())
        .collect();
}

fn task_run(
    input_path: &str,
    predicate: fn(&[i64]) -> bool,
) -> Result<i64, Box<dyn std::error::Error>> {
    let lines = read_lines_from_file(input_path);

    Ok(lines
        .map(|l| {
            let line = l.unwrap();
            let nums = i64_vec_from_line(line);
            if predicate(nums.as_slice()) {
                1
            } else {
                0
            }
        })
        .sum())
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    task_run(input_path, is_safe_v1)
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    task_run(input_path, is_safe_v2)
}

pub fn task1() -> Result<i64, Box<dyn std::error::Error>> {
    task1_run("data/day2.txt")
}

pub fn task2() -> Result<i64, Box<dyn std::error::Error>> {
    task2_run("data/day2_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_test_data() {
        assert_eq!(2, task1_run("data/day2_test.txt").unwrap())
    }

    #[test]
    fn task1() {
        assert_eq!(549, task1_run("data/day2.txt").unwrap())
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(4, task2_run("data/day2_test.txt").unwrap())
    }

    #[test]
    fn task2() {
        assert_eq!(589, task2_run("data/day2.txt").unwrap())
    }
}

use crate::file_utils::read_lines_from_file;

fn is_monotonic(nums: &[i64]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            decreasing = false;
        } else if nums[i] < nums[i - 1] {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

// Slowly - diff between each neighbours must be between 1 & 3.
fn is_slowly_increasing(nums: &[i64]) -> bool {
    for i in 1..nums.len() {
        let prev = nums[i - 1];
        let next = nums[i];
        let diff = (prev - next).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

fn is_safe(vec: Vec<i64>) -> bool {
    is_monotonic(vec.as_slice()) & is_slowly_increasing(vec.as_slice())
}

fn i64_vec_from_line(line: String) -> Vec<i64> {
    return line
        .split_whitespace()
        .map(|raw| raw.parse().unwrap())
        .collect();
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines = read_lines_from_file(input_path);

    Ok(lines
        .map(|l| {
            let line = l.unwrap();
            let nums = i64_vec_from_line(line);
            if is_safe(nums) {
                1
            } else {
                0
            }
        })
        .sum())
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    panic!("not implemented")
}

pub fn task1() -> Result<i64, Box<dyn std::error::Error>> {
    task1_run("data/day2.txt")
}

pub fn task2() -> Result<i64, Box<dyn std::error::Error>> {
    task2_run("data/day2.txt")
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
    #[ignore]
    fn task2_test_data() {
        assert_eq!(31, task2_run("data/day2_test.txt").unwrap())
    }

    #[test]
    #[ignore]
    fn task2() {
        assert_eq!(24931009, task2_run("data/day2.txt").unwrap())
    }
}

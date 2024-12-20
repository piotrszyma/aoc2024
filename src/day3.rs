use crate::file_utils::read_lines_from_file;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct MulOp {
    left: i64,
    right: i64,
}

fn find_mul_in_line(haystack: &str) -> Vec<MulOp> {
    let re_mul = Regex::new(r"mul\((?P<left_op>\d{1,3}),(?P<right_op>\d{1,3})\)").unwrap();

    let captures = re_mul.captures_iter(&haystack);

    let result: Vec<_> = captures
        .map(|c| {
            let left = c
                .name("left_op")
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

            let right = c
                .name("right_op")
                .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

            MulOp { left, right }
        })
        .collect();

    result
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let result = read_lines_from_file(input_path)
        .map(|l| {
            find_mul_in_line(&l.unwrap())
                .iter()
                .map(|v| v.left * v.right)
                .sum::<i64>()
        })
        .sum();
    Ok(result)
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    panic!("not impl")
}

pub fn task1() -> Result<i64, Box<dyn std::error::Error>> {
    task1_run("data/day3_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn std::error::Error>> {
    task2_run("data/day2_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mul_in_line_test() {
        assert_eq!(
            vec![
                MulOp { left: 1, right: 2 },
                MulOp {
                    left: 123,
                    right: 456
                },
            ],
            find_mul_in_line("mul(1,2)mul(123,456)")
        );

        assert_eq!(
            Vec::<MulOp>::new(),
            find_mul_in_line("mul(4*, mul(6,9!, ?(12,34)")
        );

        assert_eq!(Vec::<MulOp>::new(), find_mul_in_line("mul ( 2 , 4 )"));
        assert_eq!(
            vec![
                MulOp { left: 2, right: 4 },
                MulOp { left: 5, right: 5 },
                MulOp { left: 11, right: 8 },
                MulOp { left: 8, right: 5 },
            ],
            find_mul_in_line(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );
    }

    #[test]
    fn task1_test_data() {
        assert_eq!(161, task1_run("data/day3_test.txt").unwrap())
    }

    #[test]
    fn task1() {
        assert_eq!(159833790, task1_run("data/day3.txt").unwrap())
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

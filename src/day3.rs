use crate::file_utils::read_lines_from_file;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Op {
    Enable,
    Disable,
    Mul { left: i64, right: i64 },
}

fn find_op_in_line(haystack: &str) -> Vec<Op> {
    let re_mul =
        Regex::new(r"mul\((?P<left_op>\d{1,3}),(?P<right_op>\d{1,3})\)|(do(n't)?\(\))").unwrap();

    let captures = re_mul.captures_iter(&haystack);

    let result: Vec<_> = captures
        .map(|c| {
            let full_match = c.get(0);

            match full_match.unwrap().as_str() {
                "don't()" => Op::Disable,
                "do()" => Op::Enable,
                _ => {
                    let left = c
                        .name("left_op")
                        .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

                    let right = c
                        .name("right_op")
                        .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

                    Op::Mul { left, right }
                }
            }
        })
        .collect();

    result
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let result = read_lines_from_file(input_path)
        .map(|l| {
            find_op_in_line(&l.unwrap())
                .iter()
                .map(|v| match v {
                    Op::Mul { left, right } => left * right,
                    _ => 0,
                })
                .sum::<i64>()
        })
        .sum();
    Ok(result)
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let result = read_lines_from_file(input_path)
        .map(|l| {
            let ops = find_op_in_line(&l.unwrap());

            let mut enabled = true;
            let mut total = 0;

            for op in ops {
                match op {
                    Op::Disable => {
                        enabled = false;
                    }
                    Op::Enable => {
                        enabled = true;
                    }
                    Op::Mul { left, right } => {
                        if enabled {
                            total += left * right
                        }
                    }
                }
            }

            total
        })
        .sum();
    Ok(result)
}

pub fn task1() -> Result<i64, Box<dyn std::error::Error>> {
    task1_run("data/day3_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn std::error::Error>> {
    task2_run("data/day3_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_op_in_line_mul_op_test() {
        assert_eq!(
            vec![
                Op::Mul { left: 1, right: 2 },
                Op::Mul {
                    left: 123,
                    right: 456,
                },
            ],
            find_op_in_line("mul(1,2)mul(123,456)")
        );

        assert_eq!(
            Vec::<Op>::new(),
            find_op_in_line("mul(4*, mul(6,9!, ?(12,34)")
        );

        assert_eq!(Vec::<Op>::new(), find_op_in_line("mul ( 2 , 4 )"));
        assert_eq!(
            vec![
                Op::Mul { left: 2, right: 4 },
                Op::Mul { left: 5, right: 5 },
                Op::Mul { left: 11, right: 8 },
                Op::Mul { left: 8, right: 5 },
            ],
            find_op_in_line(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );
    }

    #[test]
    fn find_mul_in_line_disable_op_test() {
        assert_eq!(Vec::<Op>::new(), find_op_in_line("don't"));
        assert_eq!(Vec::<Op>::new(), find_op_in_line("dodon'tdo"));
        assert_eq!(vec![Op::Disable], find_op_in_line("don't()"));
        assert_eq!(
            vec![Op::Disable, Op::Enable, Op::Disable],
            find_op_in_line("don't()do()don'tdon't()")
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
        assert_eq!(48, task2_run("data/day3_test2.txt").unwrap())
    }

    #[test]
    fn task2() {
        // too high
        assert_eq!(91634027, task2_run("data/day3.txt").unwrap())
    }
}

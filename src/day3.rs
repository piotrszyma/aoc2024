use std::fs::read_to_string;

use regex::Regex;

// TODO: drop `pos` attribute.
#[derive(Debug, PartialEq)]
enum Op {
    Enable { pos: usize },
    Disable { pos: usize },
    Mul { left: i64, right: i64, pos: usize },
}

fn find_ops_in_line(haystack: &str) -> Vec<Op> {
    let re_mul =
        Regex::new(r"mul\((?P<left_op>\d{1,3}),(?P<right_op>\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let captures = re_mul.captures_iter(&haystack);

    let result: Vec<_> = captures
        .map(|c| {
            let full_match = c.get(0).unwrap();
            let start_pos = full_match.start();

            match full_match.as_str() {
                "don't()" => Op::Disable { pos: start_pos },
                "do()" => Op::Enable { pos: start_pos },
                _ => {
                    let left = c
                        .name("left_op")
                        .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

                    let right = c
                        .name("right_op")
                        .map_or(0, |m| m.as_str().parse::<i64>().unwrap());

                    Op::Mul {
                        left,
                        right,
                        pos: start_pos,
                    }
                }
            }
        })
        .collect();

    result
}

fn task1_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path).unwrap();
    let result = find_ops_in_line(&data)
        .iter()
        .map(|v| match v {
            Op::Mul {
                left,
                right,
                pos: _,
            } => left * right,
            _ => 0,
        })
        .sum::<i64>();
    Ok(result)
}

fn task2_run(input_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path).unwrap();
    let ops = find_ops_in_line(&data);

    let mut enabled = true;
    let mut total = 0;

    for op in ops {
        match op {
            Op::Disable { pos: _ } => {
                enabled = false;
            }
            Op::Enable { pos: _ } => {
                enabled = true;
            }
            Op::Mul {
                left,
                right,
                pos: _,
            } => {
                if enabled {
                    total += left * right
                }
            }
        }
    }

    Ok(total)
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
                Op::Mul {
                    left: 1,
                    right: 2,
                    pos: 0,
                },
                Op::Mul {
                    left: 123,
                    right: 456,
                    pos: 8,
                },
            ],
            find_ops_in_line("mul(1,2)mul(123,456)")
        );

        assert_eq!(
            Vec::<Op>::new(),
            find_ops_in_line("mul(4*, mul(6,9!, ?(12,34)")
        );

        assert_eq!(Vec::<Op>::new(), find_ops_in_line("mul ( 2 , 4 )"));
        assert_eq!(
            vec![
                Op::Mul {
                    left: 2,
                    right: 4,
                    pos: 1
                },
                Op::Mul {
                    left: 5,
                    right: 5,
                    pos: 29
                },
                Op::Mul {
                    left: 11,
                    right: 8,
                    pos: 53
                },
                Op::Mul {
                    left: 8,
                    right: 5,
                    pos: 62
                },
            ],
            find_ops_in_line(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );
    }

    #[test]
    fn find_op_in_line_disable_enable_test() {
        assert_eq!(Vec::<Op>::new(), find_ops_in_line("don't"));
        assert_eq!(Vec::<Op>::new(), find_ops_in_line("do"));
        assert_eq!(Vec::<Op>::new(), find_ops_in_line("dodon'tdo"));
        assert_eq!(vec![Op::Disable { pos: 0 }], find_ops_in_line("don't()"));
        assert_eq!(vec![Op::Enable { pos: 0 }], find_ops_in_line("do()"));
        assert_eq!(
            vec![
                Op::Disable { pos: 0 },
                Op::Enable { pos: 7 },
                Op::Disable { pos: 16 }
            ],
            find_ops_in_line("don't()do()don'tdon't()")
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
        assert_eq!(89349241, task2_run("data/day3.txt").unwrap());
    }
}

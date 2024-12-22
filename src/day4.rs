use std::error::Error;

use crate::file_utils::read_lines_from_file_v2;

fn row_chars_to_i64(rows: &Vec<String>) -> Vec<Vec<i64>> {
    rows.iter()
        .map(|r| {
            r.chars()
                .into_iter()
                .map(|c| match c {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn find_xmas(rows: &Vec<Vec<i64>>, start: (i64, i64), diff: (i64, i64)) -> i64 {
    let mut pos = start;
    let mut expected = 1;

    for _ in 0..=3 {
        let row_idx: usize = match pos.0.try_into() {
            Ok(x) => x,
            Err(_) => return 0,
        };

        let col_idx: usize = match pos.1.try_into() {
            Ok(x) => x,
            Err(_) => return 0,
        };

        let cell = match rows.get(row_idx) {
            Some(x) => match x.get(col_idx) {
                Some(v) => *v,
                None => return 0,
            },
            None => return 0,
        };

        if cell == expected {
            expected += 1;
        } else {
            return 0;
        }

        pos.0 += diff.0;
        pos.1 += diff.1;
    }

    return 1;
}

fn count_xmas(rows: &Vec<Vec<i64>>) -> i64 {
    let mut total = 0;

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell != 1i64 {
                continue;
            }

            let row_idx = row_idx.try_into().unwrap();
            let col_idx = col_idx.try_into().unwrap();

            for row_diff in -1..=1 {
                for col_diff in -1..=1 {
                    total += find_xmas(rows, (row_idx, col_idx), (row_diff, col_diff));
                }
            }
        }
    }

    total
}

fn get_at_point(rows: &Vec<Vec<i64>>, coord: (i64, i64)) -> Option<i64> {
    let x: usize = match coord.0.try_into() {
        Ok(v) => v,
        Err(_) => return None,
    };

    let y: usize = match coord.1.try_into() {
        Ok(v) => v,
        Err(_) => return None,
    };

    let row = match rows.get(x) {
        Some(row) => row,
        None => return None,
    };

    match row.get(y) {
        Some(value) => Some(*value),
        None => return None,
    }
}

fn find_xmas_v2(rows: &Vec<Vec<i64>>, mid: (i64, i64)) -> i64 {
    let left_top = get_at_point(rows, (mid.0 - 1, mid.1 + 1)).unwrap_or(-1);
    let right_bottom = get_at_point(rows, (mid.0 + 1, mid.1 - 1)).unwrap_or(-1);

    let left_bottom = get_at_point(rows, (mid.0 - 1, mid.1 - 1)).unwrap_or(-1);
    let right_top = get_at_point(rows, (mid.0 + 1, mid.1 + 1)).unwrap_or(-1);

    let has_left_top_right_bottom_xmas = match (left_top, right_bottom) {
        (2, 4) => true,
        (4, 2) => true,
        _ => false,
    };

    let has_right_top_left_bottom_xmas = match (right_top, left_bottom) {
        (2, 4) => true,
        (4, 2) => true,
        _ => false,
    };

    if has_left_top_right_bottom_xmas && has_right_top_left_bottom_xmas {
        1
    } else {
        0
    }
}

fn count_xmas_v2(rows: &Vec<Vec<i64>>) -> i64 {
    let mut total = 0;

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell != 3i64 {
                continue;
            }

            let row_idx = row_idx.try_into().unwrap();
            let col_idx = col_idx.try_into().unwrap();

            total += find_xmas_v2(rows, (row_idx, col_idx));
        }
    }

    total
}

type SearchFn = fn(&Vec<Vec<i64>>) -> i64;

fn task_run(path: &str, count_fn: SearchFn) -> Result<i64, Box<dyn Error>> {
    let rows: Vec<_> = read_lines_from_file_v2(path).collect();
    let rows = row_chars_to_i64(&rows);
    let count = count_fn(&rows);
    Ok(count)
}

fn task1_run(path: &str) -> Result<i64, Box<dyn Error>> {
    task_run(path, count_xmas)
}

fn task2_run(path: &str) -> Result<i64, Box<dyn Error>> {
    task_run(path, count_xmas_v2)
}

pub fn task1() -> Result<i64, Box<dyn Error>> {
    task1_run("data/day4_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn Error>> {
    task2_run("data/day4_test.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_needle_2d_xmas_test() {
        let rows = vec![
            "....XXMAS.",
            ".SAMXMS...",
            "...S..A...",
            "..A.A.MS.X",
            "XMASAMX.MM",
            "X.....XA.A",
            "S.S.S.S.SS",
            ".A.A.A.A.A",
            "..M.M.M.MM",
            ".X.X.XMASX",
        ];

        assert_eq!(
            18,
            count_xmas(&row_chars_to_i64(
                &rows.into_iter().map(|v| v.to_string()).collect()
            ))
        )
    }

    #[test]
    fn task1_test_data_test() {
        assert_eq!(18, task1_run("data/day4_test.txt").unwrap())
    }

    #[test]
    fn task1_test() {
        assert_eq!(2662, task1_run("data/day4.txt").unwrap())
    }

    #[test]
    fn task2_test_data_test() {
        assert_eq!(9, task2_run("data/day4_test.txt").unwrap())
    }

    #[test]
    fn task2_test() {
        assert_eq!(2034, task2_run("data/day4.txt").unwrap())
    }
}

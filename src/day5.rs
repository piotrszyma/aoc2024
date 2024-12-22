use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
};

use crate::file_utils::read_lines_from_file_v2;

#[derive(Debug, PartialEq)]
struct OrderingRule {
    value: i64,
    value_must_be_before: i64,
}

type OrderingRules = HashMap<i64, Vec<i64>>;

#[derive(Debug, PartialEq)]
struct PageToProduce {
    values: Vec<i64>,
}

impl PageToProduce {
    fn from(line: &str) -> Result<Self, std::num::ParseIntError> {
        let values = line
            .split(',')
            .map(str::trim) // clean up whitespace
            .map(str::parse) // parse each value
            .collect::<Result<Vec<i64>, _>>()?; // collect Results into a Result<Vec>

        Ok(PageToProduce { values })
    }

    fn to_sorted(&self, ordering_rules: &OrderingRules) -> PageToProduce {
        let mut sorted_values = self.values.clone();

        sorted_values.sort_by(|&a, &b| {
            ordering_rules
                .get(&a)
                .map_or(false, |deps| deps.contains(&b))
                .then_some(Ordering::Greater)
                .or_else(|| {
                    ordering_rules
                        .get(&b)
                        .map_or(false, |deps| deps.contains(&a))
                        .then_some(Ordering::Less)
                })
                .unwrap_or(Ordering::Equal)
        });

        PageToProduce{values: sorted_values}
    }

}

#[derive(Debug, PartialEq)]
struct InputData {
    /// Maps a value to a list of values that must be before it.
    ordering_rules: HashMap<i64, Vec<i64>>,
    pages: Vec<PageToProduce>,
}

impl InputData {

    #[cfg(test)]
    pub fn from_slice(lines: Vec<&str>) -> Result<InputData, std::num::ParseIntError> {
        Self::from(lines.into_iter().map(|l| l.to_string()).collect())
    }

    #[cfg(test)]
    pub fn from(lines: Vec<String>) -> Result<InputData, std::num::ParseIntError> {
        Self::from_iter(lines.into_iter())
    }

    pub fn from_iter(
        lines: impl Iterator<Item = String>,
    ) -> Result<InputData, std::num::ParseIntError> {
        let mut rules: Vec<OrderingRule> = Vec::new();
        let mut pages: Vec<PageToProduce> = Vec::new();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            match line.split_once('|') {
                Some((left, right)) => rules.push(OrderingRule {
                    value_must_be_before: left.parse()?,
                    value: right.parse()?,
                }),
                None => pages.push(PageToProduce::from(&line)?),
            }
        }

        let ordering_rules = {
            let mut ordering_rules: HashMap<i64, Vec<i64>> = HashMap::new();

            for rule in rules {
                ordering_rules
                    .entry(rule.value)
                    .or_insert(Vec::new())
                    .push(rule.value_must_be_before)
            }

            ordering_rules
        };

        Ok(InputData {
            ordering_rules,
            pages,
        })
    }
}

fn is_valid_page_to_produce(page: &PageToProduce, ordering_rules: &HashMap<i64, Vec<i64>>) -> bool {
    return page == &page.to_sorted(ordering_rules)
}

fn filter_valid_pages_to_produce(input_data: InputData) -> impl Iterator<Item = PageToProduce> {
    input_data.pages.into_iter().filter_map(move |page| {
        if is_valid_page_to_produce(&page, &input_data.ordering_rules) {
            Some(page)
        } else {
            None
        }
    })
}

fn sum_middle_item_of_pages(pages: impl Iterator<Item = PageToProduce>) -> i64 {
    pages
        .map(|p| {
            let mid = p.values.len() / 2;
            p.values[mid]
        })
        .sum()
}

fn task1_run(path: &str) -> Result<i64, Box<dyn Error>> {
    let lines = read_lines_from_file_v2(path);
    let input_data = InputData::from_iter(lines)?;
    let valid_pages = filter_valid_pages_to_produce(input_data);
    Ok(sum_middle_item_of_pages(valid_pages))
}

fn fix_invalid_pages(input_data: InputData) -> Vec<PageToProduce> {
    input_data
        .pages
        .into_iter()
        .filter(|page| !is_valid_page_to_produce(page, &input_data.ordering_rules))
        .map(|page| page.to_sorted(&input_data.ordering_rules))
        .collect()
}

fn task2_run(path: &str) -> Result<i64, Box<dyn Error>> {
    let lines = read_lines_from_file_v2(path);
    let input_data = InputData::from_iter(lines)?;
    let invalid_pages_fixed = fix_invalid_pages(input_data);
    Ok(sum_middle_item_of_pages(invalid_pages_fixed.into_iter()))
}

pub fn task1() -> Result<i64, Box<dyn Error>> {
    task1_run("data/day5_test.txt")
}

pub fn task2() -> Result<i64, Box<dyn Error>> {
    task2_run("data/day5_test.txt")
}



#[cfg(test)]
mod tests {
    use super::*;


const TEST_DATA: &'static [&'static str] = &[
    "47|53",
    "97|13",
    "97|61",
    "97|47",
    "75|29",
    "61|13",
    "75|53",
    "29|13",
    "97|29",
    "53|29",
    "61|53",
    "97|53",
    "61|29",
    "47|13",
    "75|47",
    "97|75",
    "47|61",
    "75|61",
    "47|29",
    "75|13",
    "53|13",
    "",
    "75,47,61,53,29",
    "97,61,53,29,13",
    "75,29,13",
    "75,97,47,61,53",
    "61,13,29",
    "97,13,75,29,47",
];

    #[test]
    fn parse_input_test() {
        let lines = vec!["11|12", "", "11,12"];

        let result = InputData::from_slice(lines).unwrap();

        assert_eq!(
            result.pages,
            vec![PageToProduce {
                values: vec![11, 12]
            },]
        );

        assert_eq!(result.ordering_rules.get(&12), Some(&vec![11]),);
    }

    #[test]
    fn filter_valid_pages_to_produce_test() {
        let input_data = InputData::from_slice(TEST_DATA.to_vec()).unwrap();

        let result: Vec<_> = filter_valid_pages_to_produce(input_data).collect();

        assert_eq!(
            vec![
                PageToProduce::from("75,47,61,53,29").unwrap(),
                PageToProduce::from("97,61,53,29,13").unwrap(),
                PageToProduce::from("75,29,13").unwrap(),
            ],
            result,
        )
    }

    #[test]
    fn sum_middle_item_of_pages_test() {
        let pages = vec![
            PageToProduce::from("75,47,61,53,29").unwrap(),
            PageToProduce::from("97,61,53,29,13").unwrap(),
            PageToProduce::from("75,29,13").unwrap(),
        ];

        let a_sum = sum_middle_item_of_pages(pages.into_iter());

        assert_eq!(143, a_sum);
    }

    #[test]
    fn fix_invalid_pages_test() {
        let input_data = InputData::from_slice(TEST_DATA.to_vec()).unwrap();

        let result = fix_invalid_pages(input_data);

        assert_eq!(
            vec![
                PageToProduce::from("97,75,47,61,53").unwrap(),
                PageToProduce::from("61,29,13").unwrap(),
                PageToProduce::from("97,75,47,29,13").unwrap(),
            ],
            result,
        )
    }

    #[test]
    fn task1_test_data_test() {
        assert_eq!(143, task1_run("data/day5_test.txt").unwrap())
    }

    #[test]
    fn task1_test() {
        assert_eq!(5374, task1_run("data/day5.txt").unwrap())
    }

    #[test]
    fn task2_test_data_test() {
        assert_eq!(123, task2_run("data/day5_test.txt").unwrap())
    }

    #[test]
    fn task2_test() {
        assert_eq!(4260, task2_run("data/day5.txt").unwrap())
    }
}

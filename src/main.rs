use std::env;

use task_registry::TaskRegistry;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod file_utils;
mod task_registry;

fn main() {
    let tasks_registry = TaskRegistry::new(&[
        ("day1_task1", day1::task1),
        ("day1_task2", day1::task2),
        ("day2_task1", day2::task1),
        ("day2_task2", day2::task2),
        ("day3_task1", day3::task1),
        ("day3_task2", day3::task2),
        ("day4_task1", day4::task1),
        ("day4_task2", day4::task2),
        ("day5_task1", day5::task1),
        ("day5_task2", day5::task2),
        ("day6_task1", day6::task1),
        ("day6_task2", day6::task2),
        ("day7_task1", day7::task1),
        ("day7_task2", day7::task2),
    ]);

    let task_id = match env::args().nth(1) {
        Some(task_id) => task_id,
        _ => tasks_registry.latest_task_id().to_string(),
    };

    let result = match tasks_registry.get(&task_id) {
        Some(func) => {
            println!("Running task_id: {}", task_id);
            func()
        }
        _ => panic!("Invalid task_id: {}", task_id),
    };

    if let Ok(value) = result {
        println!("Result: {}", value);
    } else {
        panic!("Error: {:?}", result.unwrap_err());
    }
}

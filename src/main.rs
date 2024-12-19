use std::env;

use task_registry::TaskRegistry;

mod day1;
mod day2;

mod file_utils;
mod task_registry;

fn main() {
    let tasks_registry = TaskRegistry::new(&[
        ("day1_task1", day1::task1),
        ("day1_task2", day1::task2),
        ("day2_task1", day2::task1),
        ("day2_task2", day2::task2),
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

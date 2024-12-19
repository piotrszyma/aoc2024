use std::{collections::HashMap, error::Error};

pub type Task = fn() -> Result<i64, Box<dyn Error>>;

pub struct TaskRegistry {
    tasks: HashMap<String, Task>,
    latest_task_id: String,
}

impl TaskRegistry {
    pub fn new(tasks_init: &[(&str, Task)]) -> Self {
        let mut tasks = HashMap::<String, Task>::new();
        for (task_id, task_fn) in tasks_init {
            tasks.insert(task_id.to_string(), *task_fn);
        }
        let latest_task_id = tasks_init
            .last()
            .expect("tasks_init should have at least one task")
            .0
            .to_string();
        TaskRegistry {
            tasks,
            latest_task_id,
        }
    }

    pub fn get(&self, task_id: &str) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    pub fn latest_task_id(&self) -> &str {
        &self.latest_task_id
    }
}

use std::collections::HashMap;
use std::io::{self, Write};

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(
            id,
            Task {
                id,
                description,
                completed: false,
            },
        );
        self.next_id += 1;
        id
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.completed = true;
                Ok(())
            }
            None => Err(format!("Could not find task with id: {}", id)),
        }
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            println!(
                "Task {}: {} {}",
                task.id,
                task.description,
                if task.completed { "✔" } else { "✘" }
            );
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if self.tasks.contains_key(&id) {
            self.tasks.remove(&id);
            Ok(())
        } else {
            Err(format!("Remove task with id {} failed.", id))
        }
    }
}

fn add_task(task_manager: &mut TaskManager) {
    println!("Enter task description: ");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read ling.");
    task_manager.add_task(description.trim().to_string());
    println!("Task added successfully.");
}

fn complete_task(task_manager: &mut TaskManager) {
    println!("Enter task ID to complete: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    match id.trim().parse() {
        Ok(id) => match task_manager.complete_task(id) {
            Ok(_) => println!("Task completed successfully."),
            Err(e) => println!("Error: {}", e),
        },
        Err(_) => println!("Invalid ID format."),
    }
}

fn list_tasks(task_manager: &mut TaskManager) {
    println!("Tasks: ");
    task_manager.list_tasks();
}

fn delete_task(task_manager: &mut TaskManager) {
    println!("Enter task ID to delete: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read ling");
    match id.trim().parse() {
        Ok(id) => match task_manager.delete_task(id) {
            Ok(_) => println!("Task {} deleted successfully.", id),
            Err(e) => println!("Error: {}", e),
        },
        Err(_) => println!("Invalid ID format."),
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("\nWelcome to task manager:");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. List tasks");
        println!("4. Delete task");
        println!("5. Exit");
        print!("Choose your option: ");
        io::stdout().flush().unwrap();
        // - `io::stdout().flush()` 确保输出立即显示，这在使用 `print!` 而不是 `println!` 时特别重要。

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");
        // `read_line()` 方法会保留输入中的换行符，这就是为什么我们经常需要使用 `trim()`。
        // `parse()` 方法尝试将字符串解析为指定类型。它返回一个 `Result`，这就是为什么我们使用 `match` 或 `if let` 来处理结果。
        let choice = input.trim().parse().unwrap_or(0);

        match choice {
            1 => add_task(&mut task_manager),
            2 => complete_task(&mut task_manager),
            3 => list_tasks(&mut task_manager),
            4 => delete_task(&mut task_manager),
            5 => break,
            _ => println!("Invalid option"),
        }
    }
}

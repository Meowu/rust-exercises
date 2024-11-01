use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
// use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    Complete { id: usize },
    List,
    Delete { id: usize },
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            created_at: Utc::now(),
        };
        self.tasks.push(task);
        println!("Task added with ID: {}", self.next_id);
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task {} marked as completed.", id);
        } else {
            eprintln!("Task not found.");
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            let description = if task.completed {
                // shared borrow occurs here.
                text_to_strikethrough(&task.description)
            } else {
                task.description.clone()
            };
            println!(
                "{:>3}. [{}] {} (Created: {})",
                task.id,
                if task.completed { "x" } else { " " },
                description,
                task.created_at.format("%y-%m-%d %H:%M:%S")
            )
        }
    }

    fn delete_task(&mut self, id: usize) {
        self.tasks.retain(|t| t.id != id);
        println!("Task {} deleted.", id);
    }
}

// be sure to prepend `\x1b` to [0m to reset the strikethrough style.
fn text_to_strikethrough(text: &str) -> String {
    format!("\x1b[9m{}\x1b[0m", text)
}

// works in webpage or editor.
// fn text_to_strikethrough(text: &str) -> String {
//     text.chars().map(|c| format!("{}\u{0336}", c)).collect()
// }

fn load_tasks(path_str: &str) -> TaskManager {
    let path = Path::new(path_str);
    if !path.exists() {
        return TaskManager::new();
    }

    let mut file = File::open(path).expect("Unable to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file.");

    let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    TaskManager { tasks, next_id }
}

fn save_tasks(manager: &TaskManager, path_str: &str) {
    let serialized = serde_json::to_string_pretty(&manager.tasks).expect("Unable to serialize");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path_str)
        .expect("Unable to open file.");
    file.write_all(serialized.as_bytes())
        .expect("Unable to write file");
}

fn main() {
    println!("Welcome to TODO manager!");
    // CARGO_MANIFEST_DIR 是包的根目录
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // 构建任务文件在源代码目录下的路径
    let file_path = Path::new(manifest_dir).join("src").join("tasks.json");
    println!("current dir: {:}", file_path.to_str().unwrap());
    let path_str = file_path.to_str().unwrap();
    let cli = Cli::parse();
    let mut manager = load_tasks(path_str);

    match &cli.command {
        Some(Commands::Add { description }) => {
            manager.add_task(description.clone());
        }
        Some(Commands::Complete { id }) => {
            manager.complete_task(*id);
        }
        Some(Commands::List) => {
            manager.list_tasks();
        }
        Some(Commands::Delete { id }) => {
            manager.delete_task(*id);
        }
        None => {
            println!("No command specified. Use --help for usage infomation.")
        }
    }
    save_tasks(&manager, path_str);
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "todo")]
enum Opt {
    #[structopt(name = "add")]
    Add { description: String },

    #[structopt(name = "complete")]
    Complete { id: usize },

    #[structopt(name = "list")]
    List,

    #[structopt(name = "delete")]
    Delete { id: usize },
}

fn load_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");
    if !path.exists() {
        return Vec::new();
    }

    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &[Task]) {
    let serialized = serde_json::to_string_pretty(tasks).expect("Unable to serialize");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Unable to open file");

    file.write_all(serialized.as_bytes())
        .expect("Unable to write file");
}

fn add_task(description: String, tasks: &mut Vec<Task>) {
    let id = tasks.len() + 1;
    let task = Task {
        id,
        description,
        completed: false,
        created_at: Utc::now(),
    };
    tasks.push(task);
    println!("Task added with ID: {}", id);
}

fn complete_task(id: usize, tasks: &mut Vec<Task>) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task {} marked as completed", id);
    } else {
        println!("Task not found");
    }
}

fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!(
            "{}. [{}] {} (Created: {})",
            task.id,
            if task.completed { "x" } else { " " },
            task.description,
            task.created_at.format("%Y-%m-%d %H:%M:%S")
        );
    }
}

fn delete_task(id: usize, tasks: &mut Vec<Task>) {
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        println!("Task {} deleted", id);
    } else {
        println!("Task not found");
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut tasks = load_tasks();

    match opt {
        Opt::Add { description } => add_task(description, &mut tasks),
        Opt::Complete { id } => complete_task(id, &mut tasks),
        Opt::List => list_tasks(&tasks),
        Opt::Delete { id } => delete_task(id, &mut tasks),
    }

    save_tasks(&tasks);
}

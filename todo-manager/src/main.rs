use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::process::Command;
// use structopt::StructOpt;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

    fn enable_raw_mode() {
        if cfg!(unix) {
            Command::new("stty")
                .arg("raw")
                .arg("-echo")
                .spawn()
                .expect("Failed to enable raw mode");
        }
    }

    fn disable_raw_mode() {
        if cfg!(unix) {
            Command::new("stty")
                .arg("-raw")
                .arg("echo")
                .spawn()
                .expect("Failed to disable raw mode");
        }
    }

    fn interactive_list(&mut self) -> std::io::Result<()> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout().into_raw_mode()?;
        let mut selected = 0;

        for key in stdin.keys() {
            print!("{}", termion::clear::All);
            print!("{}", termion::cursor::Goto(1, 1));

            // 显示任务列表
            for (index, task) in self.tasks.iter().enumerate() {
                if index == selected {
                    print!("> "); // 高亮当前选中项
                } else {
                    print!("  ");
                }

                println!(
                    "[{}] {}",
                    if task.completed { "x" } else { " " },
                    task.description
                );
            }

            match key? {
                Key::Char('q') => break,
                Key::Char(' ') => {
                    if let Some(task) = self.tasks.get_mut(selected) {
                        task.completed = !task.completed;
                    }
                }
                Key::Up if selected > 0 => selected -= 1,
                Key::Down if selected < self.tasks.len() - 1 => selected += 1,
                _ => {}
            }
        }
        Ok(())
    }

    // 主要使用的 ANSI 转义序列：
    // - `\x1B[2J\x1B[1;1H`: 清屏并移动光标到左上角
    // - `\x1B[7m`: 反转颜色
    // - `\x1B[0m`: 重置样式
    // - `\x1B[s`: 保存光标位置
    // - `\x1B[u`: 恢复光标位置
    // - `\x1B[{n};1H`: 移动光标到指定行
    fn interactive_list_raw(&mut self) -> std::io::Result<()> {
        if self.tasks.is_empty() {
            println!("No tasks.");
            return Ok(());
        }
        Self::enable_raw_mode();
        let mut selected = 0;
        let mut input = ' ';
        let mut stdout = stdout();
        let mut stdin = stdin();

        loop {
            // 清屏
            // print!("\x1B[2J\x1B[1;1H");
            // 清屏
            print!("\x1B[2J");
            // 移动光标到开始位置
            print!("\x1B[H");
            stdout.flush()?;

            // 显示任务
            for (index, task) in self.tasks.iter().enumerate() {
                // if index == selected {
                //     // 保存光标位置
                //     print!("\x1B[s");
                //     // 启用鼠标跟踪
                //     print!("\x1B[?1000h");
                //     // 反转颜色
                //     print!("\x1B[7m");
                // }

                let description = if task.completed {
                    // shared borrow occurs here.
                    text_to_strikethrough(&task.description)
                } else {
                    task.description.clone()
                };
                let line = format!(
                    "{:>3}. [{}] {} (Created: {})",
                    task.id,
                    if task.completed { "x" } else { " " },
                    description,
                    task.created_at.format("%y-%m-%d %H:%M:%S")
                );

                // println!(
                //     "[{}] {}",
                //     if task.completed { "x" } else { " " },
                //     task.description
                // );

                // if index == selected {
                //     // 重置样式
                //     print!("\x1B[0m");
                //     // 恢复光标位置
                //     print!("\x1B[u");
                // }
                if index == selected {
                    // 高亮显示选中行
                    print!("\x1B[7m{}\x1B[0m\n", line);
                } else {
                    print!("{}\n", line);
                }
            }

            println!("\nUse ↑↓ to navigate, Space to toggle, q to quit");
            // println!("last input: {}", input);
            stdout.flush()?;

            // 将光标移动到选中行
            print!("\x1B[{};1H", selected + 2);
            stdout.flush()?;

            // 读取按键
            let mut buffer = [0; 3];
            stdin.read_exact(&mut buffer[..1])?;

            match buffer[0] {
                b'q' => {
                    // 禁用鼠标跟踪
                    print!("\x1B[?1000l");
                    stdout.flush()?;
                    break;
                }
                b' ' => {
                    input = ' ';
                    if let Some(task) = self.tasks.get_mut(selected) {
                        task.completed = !task.completed;
                    }
                }
                b'k' => {
                    input = 'k';
                    if selected > 0 {
                        selected -= 1
                    }
                }
                b'j' => {
                    input = 'j';
                    if selected < self.tasks.len() - 1 {
                        selected += 1
                    }
                }
                27 => {
                    input = '7';
                    // ESC 键，可能是方向键的开始
                    println!("read extact.");
                    stdin.read_exact(&mut buffer[1..3])?;
                    match &buffer[..] {
                        [27, 91, 65] => {
                            // 上箭头
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        [27, 91, 66] => {
                            // 下箭头
                            if selected < self.tasks.len() - 1 {
                                selected += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Self::disable_raw_mode();
        Ok(())
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
            // manager.list_tasks();
            manager
                .interactive_list_raw()
                .expect("Failed to show interactive list");
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

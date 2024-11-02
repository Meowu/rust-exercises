use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

fn show_progress(total: u32) -> io::Result<()> {
    let mut stdout = io::stdout();

    for i in 0..=total {
        print!("\r");

        print!("Progress: [");

        for j in 1..=total {
            if j <= i {
                print!("#");
            } else {
                print!(" ");
            }
        }

        print!("] {}/{}", i, total);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(100));
    }
    println!();

    Ok(())
}

fn show_spinner() -> io::Result<()> {
    let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut stdout = io::stdout();

    for _ in 0..50 {
        for &c in &spinner {
            print!("\r{} Loading...", c);
            stdout.flush()?;
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!();
    Ok(())
}

// todo: move cursor to the start of selected line.
fn show_menu() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut selection = 0;
    let options = vec!["Option 1", "Option 2", "Option 3"];

    loop {
        // 清屏并移动光标到起始位置
        print!("\x1B[2J\x1B[H");
        stdout.flush()?;

        // 显示选项
        for (i, option) in options.iter().enumerate() {
            if i == selection {
                // 高亮显示当前选项
                print!("\x1B[7m{}\x1B[0m\n", option);
            } else {
                println!("{}", option);
            }
        }
        stdout.flush()?;

        // 读取一个字节
        let mut buffer = [0; 3];
        stdin.read_exact(&mut buffer[..1])?;

        match buffer[0] {
            b'q' => break, // 退出
            27 => {
                // ESC 键，可能是方向键
                stdin.read_exact(&mut buffer[1..3])?;
                match &buffer[..] {
                    [27, 91, 65] => {
                        // 上箭头
                        if selection > 0 {
                            selection -= 1;
                        }
                    }
                    [27, 91, 66] => {
                        // 下箭头
                        if selection < options.len() - 1 {
                            selection += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("Hello, world!\rHi");
    show_menu()
    // show_progress(24)
    // show_spinner()
}

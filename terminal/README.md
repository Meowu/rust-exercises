# Terminal

Learn how to control terminal in rust.

### 1. 基本格式

所有 ANSI 转义序列都以 `ESC[` 开头 (在 Rust 中写作 `\x1B[` 或 `\u{001B}[`)

### 2. 光标控制

`\r` 是 "Carriage Return" 的缩写，相当于机械打字机的“回车”动作，把打字头移到行首位置。
在终端环境中，`\r` 将光标移到当前行的开头位置，但不会清除行内已有的内容，也不会换到下一行。

```rust

// 移动到行首
print!("\r");

// "Hello, world!" 的起始位置被 "Hi!" 替代，因为 \r 移动了光标，使得后续的 "Hi!" 从行首开始打印。
print!("Hello, world!\rHi!"); // Hi!lo, world!

// 移动光标到指定位置 (行;列)
print!("\x1B[{};{}H", row, column);  // 例如 "\x1B[1;1H" 移动到第1行第1列

// 移动光标到屏幕起始位置
print!("\x1B[H");

// 保存光标位置
print!("\x1B[s");

// 恢复光标位置
print!("\x1B[u");
```

### 3. 清屏操作

```rust
// 清除整个屏幕
print!("\x1B[2J");

// 清除从光标到行尾
print!("\x1B[K");
```

### 4. 文本样式

```rust
// 反转颜色（背景色和前景色互换）
print!("\x1B[7m");  // 开启
print!("\x1B[27m"); // 关闭

// 重置所有属性
print!("\x1B[0m");
```

### 5. 颜色控制

```rust
// 前景色 (文字颜色)
const FG_BLACK: &str = "\x1B[30m";
const FG_RED: &str = "\x1B[31m";
const FG_GREEN: &str = "\x1B[32m";
const FG_YELLOW: &str = "\x1B[33m";
const FG_BLUE: &str = "\x1B[34m";
const FG_MAGENTA: &str = "\x1B[35m";
const FG_CYAN: &str = "\x1B[36m";
const FG_WHITE: &str = "\x1B[37m";

// 背景色
const BG_BLACK: &str = "\x1B[40m";
const BG_RED: &str = "\x1B[41m";
const BG_GREEN: &str = "\x1B[42m";
const BG_YELLOW: &str = "\x1B[43m";
const BG_BLUE: &str = "\x1B[44m";
const BG_MAGENTA: &str = "\x1B[45m";
const BG_CYAN: &str = "\x1B[46m";
const BG_WHITE: &str = "\x1B[47m";

### 6. 光标移动详解
```rust
// 光标上移 n 行
print!("\x1B[{}A", n);

// 光标下移 n 行
print!("\x1B[{}B", n);

// 光标右移 n 列
print!("\x1B[{}C", n);

// 光标左移 n 列
print!("\x1B[{}D", n);
```

### 7. 屏幕清除详解
```rust
// 从光标到屏幕末尾清除
print!("\x1B[J");

// 从光标到屏幕开头清除
print!("\x1B[1J");

// 整个屏幕清除
print!("\x1B[2J");

// 从光标到行末清除
print!("\x1B[K");

// 从光标到行首清除
print!("\x1B[1K");

// 整行清除
print!("\x1B[2K");
```

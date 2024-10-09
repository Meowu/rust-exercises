use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// 目标：创建一个程序，读取文本文件，统计单词出现的频率，并输出最常见的 N 个单词。
///
/// Usage: {} <filename> <top_n>

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <filename> <top_n>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let top_n: usize = args[2].parse()?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut word_counts: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    let mut word_counts: Vec<_> = word_counts.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top {} words: ", top_n);
    // for (word, count) in word_counts.iter().take(top_n) {
    //     println!("{}: {}", word, count);
    // }

    // optimized version:
    for (word, count) in &word_counts[..top_n.min(word_counts.len())] {
        println!("{}: {}", word, count);
    }
    Ok(())
}

// Text Analyzer CLI in Rust
// Usage: text_analyzer <file-path> [top-n]
// Shows basic text statistics and top N most frequent words.

use std::env;
use std::fs;
use std::collections::HashMap;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file-path> [top-n]", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let top_n: usize = if args.len() >= 3 {
        args[2].parse().unwrap_or(10)
    } else {
        10
    };

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let stats = analyze_text(&content);
    print_stats(&stats, top_n);
}

#[derive(Debug)]
struct TextStats {
    char_count: usize,
    char_count_no_spaces: usize,
    line_count: usize,
    word_count: usize,
    unique_word_count: usize,
    avg_word_length: f64,
    word_frequencies: Vec<(String, usize)>,
}

fn analyze_text(content: &str) -> TextStats {
    let char_count = content.chars().count();
    let char_count_no_spaces = content.chars().filter(|c| !c.is_whitespace()).count();
    let line_count = content.lines().count();

    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut total_word_len: usize = 0;
    let mut word_count: usize = 0;

    for raw_word in content.split_whitespace() {
        // Normalize word: lowercase and strip punctuation at edges
        let cleaned = raw_word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if cleaned.is_empty() {
            continue;
        }

        word_count += 1;
        total_word_len += cleaned.chars().count();
        *freq.entry(cleaned).or_insert(0) += 1;
    }

    let unique_word_count = freq.len();
    let avg_word_length = if word_count > 0 {
        total_word_len as f64 / word_count as f64
    } else {
        0.0
    };

    let mut word_frequencies: Vec<(String, usize)> =
        freq.into_iter().map(|(w, c)| (w, c)).collect();

    word_frequencies.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    TextStats {
        char_count,
        char_count_no_spaces,
        line_count,
        word_count,
        unique_word_count,
        avg_word_length,
        word_frequencies,
    }
}

fn print_stats(stats: &TextStats, top_n: usize) {
    println!("=============================");
    println!("  Text Analyzer (Rust CLI)");
    println!("=============================\n");

    println!("Lines:                 {}", stats.line_count);
    println!("Characters (with spaces):    {}", stats.char_count);
    println!("Characters (no spaces):      {}", stats.char_count_no_spaces);
    println!("Words:                        {}", stats.word_count);
    println!("Unique words:                 {}", stats.unique_word_count);
    println!("Average word length:          {:.2}", stats.avg_word_length);

    println!("\nTop {} words:", top_n);
    println!("-----------------------------");

    for (i, (word, count)) in stats.word_frequencies.iter().take(top_n).enumerate() {
        println!("{:>2}. {:<15} {}", i + 1, word, count);
    }
}

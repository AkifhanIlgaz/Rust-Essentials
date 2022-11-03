use std::collections::HashMap;

use std::{env, fs};

fn main() {
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(1);
        }
    };

    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    let mut word_counts = HashMap::new();

    for word in all_words {
        *word_counts.entry(word).or_insert(0) += 1
    }

    let mut top_count = 0u32;

    let mut top_words: Vec<&str> = Vec::new();

    for (key, val) in word_counts {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }

    println!("Top word(s) occurred {} times", top_count);

    for word in top_words {
        println!("{}", word);
    }
}

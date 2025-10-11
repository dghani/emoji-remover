use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("emoji_data.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    let emoji_json_path = "data/all-emoji.json";
    let modifier_json_path = "data/full-emoji-modifiers.json";

    writeln!(f, "pub static EMOJI_PATTERNS: &[&str] = &[").unwrap();

    let mut emojis = Vec::new();

    if Path::new(emoji_json_path).exists() {
        let content = fs::read_to_string(emoji_json_path).unwrap();
        extract_emojis(&content, &mut emojis);
    }

    if Path::new(modifier_json_path).exists() {
        let content = fs::read_to_string(modifier_json_path).unwrap();
        extract_emojis(&content, &mut emojis);
    }

    emojis.sort();
    emojis.dedup();

    for emoji in &emojis {
        writeln!(f, r#"    "{}","#, emoji).unwrap();
    }

    writeln!(f, "];").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", emoji_json_path);
    println!("cargo:rerun-if-changed={}", modifier_json_path);
}

fn extract_emojis(json_content: &str, emojis: &mut Vec<String>) {
    let data: Vec<serde_json::Value> = serde_json::from_str(json_content).unwrap();

    for item in data {
        if let Some(arr) = item.as_array() {
            if arr.len() == 4 {
                if let Some(emoji) = arr[2].as_str() {
                    emojis.push(emoji.to_string());
                }
            }
        }
    }
}

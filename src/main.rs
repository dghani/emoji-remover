use clap::Parser;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/emoji_data.rs"));

#[derive(Parser)]
#[command(name = "emoji-remover")]
#[command(version)]
#[command(author)]
#[command(about = "A fast command-line tool to remove emojis from source code files")]
#[command(long_about = "emoji-remover is a multi-threaded tool that removes emoji characters from text files.\n\nIt respects .gitignore by default and supports various programming language file extensions.")]
struct Args {
    #[arg(help = "Files or directories to process")]
    paths: Vec<PathBuf>,

    #[arg(short, long, help = "File extensions to process (e.g., rs,js,py)")]
    extensions: Option<String>,

    #[arg(short, long, help = "Perform a dry run without modifying files")]
    dry_run: bool,

    #[arg(short, long, help = "Quiet mode - only show files with changes")]
    quiet: bool,

    #[arg(long, help = "Don't respect .gitignore files")]
    no_ignore: bool,
}

fn main() {
    let args = Args::parse();
    
    let extensions: Vec<String> = args.extensions
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![
            "rs", "js", "ts", "jsx", "tsx", "py", "java", "c", "cpp", "h", "hpp",
            "go", "rb", "php", "cs", "swift", "kt", "scala", "md", "txt"
        ].into_iter().map(String::from).collect());

    let mut files_to_process = Vec::new();

    for path in &args.paths {
        if path.is_file() {
            files_to_process.push(path.clone());
        } else if path.is_dir() {
            let mut builder = WalkBuilder::new(path);
            
            if args.no_ignore {
                builder.ignore(false);
                builder.git_ignore(false);
                builder.git_global(false);
                builder.git_exclude(false);
            }
            
            for entry in builder.build().filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if extensions.contains(&ext.to_string_lossy().to_string()) {
                            files_to_process.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    let results: Vec<_> = files_to_process.par_iter()
        .filter_map(|path| process_file(path, args.dry_run, args.quiet))
        .collect();

    if !results.is_empty() {
        println!("\nSummary:");
        println!("Processed {} files with emojis", results.len());
        let total_removed: usize = results.iter().sum();
        println!("Total emojis removed: {}", total_removed);
    } else if !args.quiet {
        println!("No emojis found in any files.");
    }
}


fn process_file(path: &Path, dry_run: bool, quiet: bool) -> Option<usize> {
    let content = fs::read_to_string(path).ok()?;
    
    let mut emoji_count = 0;
    let cleaned_content = EMOJI_PATTERNS.iter()
        .fold(content.clone(), |acc, emoji| {
            // Count occurrences before replacement
            let before = acc.matches(emoji).count();
            emoji_count += before;
            
            // Replace emoji and any single trailing space
            let with_space = format!("{} ", emoji);
            let acc = if acc.contains(&with_space) {
                acc.replace(&with_space, "")
            } else {
                acc.replace(emoji, "")
            };
            acc
        });

    if emoji_count > 0 {
        if !quiet {
            println!("{}: Found {} emoji{}", 
                path.display(), 
                emoji_count, 
                if emoji_count == 1 { "" } else { "s" }
            );
        }

        if !dry_run {
            fs::write(path, cleaned_content).ok()?;
        }

        Some(emoji_count)
    } else {
        None
    }
}
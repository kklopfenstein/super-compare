use std::env;
use std::fs;
use std::path::PathBuf;

fn collect_files(dir: &PathBuf, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                let display = entry_path.strip_prefix(dir).unwrap_or(&entry_path);
                files.push(display.to_string_lossy().to_string());
            } else if entry_path.is_dir() {
                collect_files(&entry_path, files);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: super-compare <dir1> [dir2]");
        std::process::exit(1);
    }

    for arg in &args {
        let dir_path = PathBuf::from(arg);
        if dir_path.exists() {
            let mut dir_files = Vec::new();
            collect_files(&dir_path, &mut dir_files);
            dir_files.sort();
            println!("{}\n", dir_path.display());
            for file in dir_files {
                println!("  {}", file);
            }
            println!();
        }
    }
}


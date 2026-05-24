use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

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
    
    if args.len() < 2 {
        eprintln!("Usage: super-compare <dir1> [dir2]");
        std::process::exit(1);
    }

    let dir1_path = PathBuf::from(&args[0]);
    let dir2_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    // Create vectors to hold files from each directory
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    // Collect and sort files from directory 1
    if dir1_path.exists() {
        collect_files(&dir1_path, &mut vec1);
        vec1.sort();
        
        // Print directory 1 contents
        println!("{}\n", dir1_path.display());
        for file in &vec1 {
            println!("  {}", file);
        }
    }

    // Collect and sort files from directory 2 if provided
    if let Some(path) = dir2_path {
        if path.exists() {
            collect_files(&path, &mut vec2);
            vec2.sort();
            
            // Print directory 2 contents
            println!("{}\n", path.display());
            for file in &vec2 {
                println!("  {}", file);
            }
            
            // Compare the two vectors and output differences
            let hash1: HashSet<String> = HashSet::from_iter(vec1.iter().cloned());
            let hash2: HashSet<String> = HashSet::from_iter(vec2.iter().cloned());
            
            let unique_to_1: Vec<String> = hash1.difference(&hash2).cloned().collect();
            let unique_to_2: Vec<String> = hash2.difference(&hash1).cloned().collect();
            
            println!("\nDifferences:\n");
            
            if !unique_to_1.is_empty() {
                println!("Unique to {}", &dir1_path.display());
                for file in &unique_to_1 {
                    println!("  {}", file);
                }
            }
            
            if !unique_to_2.is_empty() {
                println!("\nUnique to {}", &path.display());
                for file in &unique_to_2 {
                    println!("  {}", file);
                }
            }
            
            let common: Vec<String> = hash1.intersection(&hash2).cloned().collect();
            if !common.is_empty() {
                println!("\nCommon files:");
                for file in &common {
                    println!("  {}", file);
                }
            }
        }
    }
}


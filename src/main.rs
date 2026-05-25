use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

fn collect_files(dir: &PathBuf, original_dir: &PathBuf, files: &mut Vec<(String, PathBuf, u64)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                // Strip the original directory prefix to get full relative path
                let display = entry_path.strip_prefix(original_dir)
                    .unwrap_or(&entry_path);
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    files.push((
                        display.to_string_lossy().to_string(),
                        entry_path.clone(),
                        metadata.len(),
                    ));
                }
            } else if entry_path.is_dir() {
                collect_files(&entry_path, original_dir, files);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    let compare_sizes = args.contains(&"-s".to_string());

    if args.is_empty() {
        eprintln!("Usage: super-compare [-s] <dir1> [dir2]");
        std::process::exit(1);
    }

    let mut positional_args = Vec::new();
    for arg in args {
        if arg != "-s" {
            positional_args.push(arg);
        }
    }

    let dir1_path = PathBuf::from(&positional_args[0]);
    let dir2_path = if positional_args.len() > 1 {
        Some(PathBuf::from(&positional_args[1]))
    } else {
        None
    };

    // Create vectors to hold files from each directory
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    // Collect files from directory 1
    if dir1_path.exists() {
        collect_files(&dir1_path, &dir1_path, &mut vec1);
        vec1.sort_by(|a, b| a.0.cmp(&b.0));
    }

    // Collect files from directory 2 if provided
    if let Some(path) = dir2_path {
        if path.exists() {
            collect_files(&path, &path, &mut vec2);
            vec2.sort_by(|a, b| a.0.cmp(&b.0));
            
            // Compare the two vectors and output differences
            let hash1: HashSet<String> = HashSet::from_iter(vec1.iter().map(|(name, _, _)| name.clone()));
            let hash2: HashSet<String> = HashSet::from_iter(vec2.iter().map(|(name, _, _)| name.clone()));
            
            // Find files in both directories
            let files_in_both: Vec<String> = vec1.iter()
                .filter(|(name, _, _)| hash2.contains(name))
                .map(|(name, _, _)| name.clone())
                .collect();
            
            // Differences showing removed files from dir1 (unique to dir1, not in dir2)
            let unique_to_1: Vec<(String, PathBuf, u64)> = vec1.iter()
                .filter(|(name, _, _)| !hash2.contains(name))
                .cloned()
                .collect();
            
            // Differences showing added files to dir2 (unique to dir2, not in dir1)
            let unique_to_2: Vec<(String, PathBuf, u64)> = vec2.iter()
                .filter(|(name, _, _)| !hash1.contains(name))
                .cloned()
                .collect();
            
            // Output added files with +
            for (name, _, _) in &unique_to_2 {
                println!("+ {}", name);
            }
            
            // Output removed files from dir1 with -
            for (name, _, _) in &unique_to_1 {
                println!("- {}", name);
            }
            
            // Compare sizes for files in both directories
            if compare_sizes {
                for name in &files_in_both {
                    if let (Some((_, _path1, size1)), Some((_, _path2, size2))) = (
                        vec1.iter().find(|(n, _, _)| n == name),
                        vec2.iter().find(|(n, _, _)| n == name)
                    ) {
                        if size1 != size2 {
                            println!("~ {} ({} -> {})", name, size1, size2);
                        }
                    }
                }
            }
        }
    }
}

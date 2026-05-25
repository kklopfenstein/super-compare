use std::process::Command;
use std::fs;
use std::env;
use tempfile::tempdir;

#[test]
fn test_compare_directories() {
    let temp_dir = tempdir().unwrap();
    let dir1_path = temp_dir.path().join("dir1");
    let dir2_path = temp_dir.path().join("dir2");
    
    fs::create_dir_all(&dir1_path).unwrap();
    fs::create_dir_all(&dir2_path).unwrap();

    // Create matching files
    fs::write(dir1_path.join("file1.txt"), "content1").unwrap();
    fs::write(dir2_path.join("file1.txt"), "content1").unwrap();

    // Create unique to dir1
    fs::write(dir1_path.join("unique1.txt"), "content1").unwrap();

    // Create unique to dir2
    fs::write(dir2_path.join("unique2.txt"), "content2").unwrap();

    fs::create_dir_all(dir1_path.join("nested")).unwrap();
    fs::create_dir_all(dir2_path.join("nested")).unwrap();
    
    // Create unique to dir1 in nested dir
    fs::write(dir1_path.join("nested").join("only1_nested.txt"), "only1_nested").unwrap();

    // Create unique to dir2 in nested dir
    fs::write(dir2_path.join("nested").join("only2_nested.txt"), "only2_nested").unwrap();

    let current_dir = env::current_dir().unwrap();
    let binary_path = current_dir.join("target").join("debug").join("super-compare.exe");
    
    assert!(binary_path.exists(), "Binary not found at {:?}", binary_path);

    let mut cmd = Command::new(binary_path);
    cmd.arg(&dir1_path).arg(&dir2_path);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Program output:\n{}", stdout);

    // Verify expected content in output - only differences with signs
    assert!(stdout.contains("+ nested\\only2_nested.txt"), "Expected + nested\\only2_nested.txt in output, got:\n{}", stdout);
    assert!(stdout.contains("+ unique2.txt"), "Expected + unique2.txt in output, got:\n{}", stdout);
    assert!(stdout.contains("- nested\\only1_nested.txt"), "Expected - nested\\only1_nested.txt in output, got:\n{}", stdout);
    assert!(stdout.contains("- unique1.txt"), "Expected - unique1.txt in output, got:\n{}", stdout);
    // Make sure directory1 directory path does not appear in output
    let dir1_str = dir1_path.to_string_lossy().to_string();
assert!(!stdout.contains(&dir1_str));
}

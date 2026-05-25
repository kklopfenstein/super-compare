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

#[test]
fn test_compare_directory_sizes() {
    let temp_dir = tempdir().unwrap();
    let dir1_path = temp_dir.path().join("dir1");
    let dir2_path = temp_dir.path().join("dir2");
    
    fs::create_dir_all(&dir1_path).unwrap();
    fs::create_dir_all(&dir2_path).unwrap();

    // Create files with same content (same size)
    fs::write(dir1_path.join("same_size.txt"), "content1").unwrap();
    fs::write(dir2_path.join("same_size.txt"), "content1").unwrap();

    // Create files with different content (different size)
    fs::write(dir1_path.join("diff_size.txt"), "content1").unwrap();
    fs::write(dir2_path.join("diff_size.txt"), "content1content1").unwrap();

    fs::create_dir_all(dir1_path.join("nested")).unwrap();
    fs::create_dir_all(dir2_path.join("nested")).unwrap();
    
    // Create files with different sizes in nested directory (same name, different sizes)
    fs::write(dir1_path.join("nested").join("nested_file.txt"), "only1_nested").unwrap();
    fs::write(dir2_path.join("nested").join("nested_file.txt"), "only2_nestedonly2_nestedonly2_nested").unwrap();

    let current_dir = env::current_dir().unwrap();
    let binary_path = current_dir.join("target").join("debug").join("super-compare.exe");
    
    assert!(binary_path.exists(), "Binary not found at {:?}", binary_path);

    // Test without -s flag (should not show ~)
    let mut cmd = Command::new(&binary_path);
    cmd.arg(&dir1_path).arg(&dir2_path);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Output without -s flag:\n{}", stdout);
    
    // Verify output without -s flag doesn't contain ~
    assert!(!stdout.contains("~"), "Output without -s flag should not contain ~, got:\n{}", stdout);

    // Test with -s flag (should show ~ for size differences)
    let mut cmd = Command::new(&binary_path);
    cmd.arg(&dir1_path).arg(&dir2_path).arg("-s");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Output with -s flag:\n{}", stdout);
    
    // Verify output with -s flag contains ~ for files with different sizes
    assert!(stdout.contains("~ diff_size.txt"), "Expected ~ diff_size.txt in output with -s flag, got:\n{}", stdout);
    assert!(stdout.contains("~ nested\\nested_file.txt"), "Expected ~ nested\\nested_file.txt in output with -s flag, got:\n{}", stdout);
    
    // Verify files with same content don't show ~
    assert!(!stdout.contains("~ same_size.txt"), "Should not show ~ for same_size.txt, got:\n{}", stdout);
}

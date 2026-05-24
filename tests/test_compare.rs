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

    // Create nested subdirectories with files
    fs::create_dir_all(dir1_path.join("nested")).unwrap();
    fs::create_dir_all(dir2_path.join("nested")).unwrap();
    fs::write(dir1_path.join("nested").join("nested1.txt"), "nested1").unwrap();
    fs::write(dir2_path.join("nested").join("nested1.txt"), "nested1").unwrap();

    // Unique nested in dir1
    fs::create_dir_all(dir1_path.join("unique_nested_1")).unwrap();
    fs::write(dir1_path.join("unique_nested_1").join("only1_nested.txt"), "only1_nested").unwrap();
    
    // Unique nested in dir2
    fs::create_dir_all(dir2_path.join("unique_nested_2")).unwrap();
    fs::write(dir2_path.join("unique_nested_2").join("only2_nested.txt"), "only2_nested").unwrap();
    
    // Shared deeper nested structure
    fs::create_dir_all(dir1_path.join("shared_nested").join("level1")).unwrap();
    fs::create_dir_all(dir2_path.join("shared_nested").join("level1")).unwrap();
    fs::write(dir1_path.join("shared_nested").join("level1").join("shared_nested.txt"), "shared").unwrap();
    fs::write(dir2_path.join("shared_nested").join("level1").join("shared_nested.txt"), "shared").unwrap();

    let current_dir = env::current_dir().unwrap();
    let binary_path = current_dir.join("target").join("release").join("super-compare.exe");
    
    assert!(binary_path.exists(), "Binary not found at {:?}", binary_path);

    let mut cmd = Command::new(binary_path);
    cmd.arg(&dir1_path).arg(&dir2_path);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("dir1"), "Expected dir1 in output, got:\n{}\n", stdout);
    assert!(stdout.contains("dir2"), "Expected dir2 in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("file1.txt"), "Expected file1.txt in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("unique1.txt"), "Expected unique1.txt in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("unique2.txt"), "Expected unique2.txt in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("nested1.txt"), "Expected nested directory files in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("only1_nested.txt"), "Expected unique nested dir1 in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("only2_nested.txt"), "Expected unique nested dir2 in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("shared_nested.txt"), "Expected shared nested files in output, got:\n{}\n", stdout,);
    assert!(stdout.contains("Common files:"), "Expected common files section in output, got:\n{}\n", stdout,);
}

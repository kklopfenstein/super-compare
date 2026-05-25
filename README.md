# Super-Compare

A command-line tool to compare directories and find differences between them.

## Usage

### Basic Comparison

```bash
# Compare two directories
super-compare [-s] <dir1> [dir2]
```

### Examples

```bash
# Compare two specific directories
super-compare dir1 dir2

# Compare directories with size comparison
super-compare -s dir1 dir2

# Compare directories with spaces in names
super-compare -s "My Documents" "Work Files"
```

### Example Output

```text
dir1:
  file1.txt
  unique1.txt


dir2:
  file1.txt
  unique2.txt
  file2.txt
```

## Installation

```bash
cargo build --release
```

The executable will be in `target/release/super-compare.exe`

## Features

- Compares contents of two directories
- Detects unique files in each directory
- Shows files that exist in both directories
- Works with nested subdirectories
- Compares file sizes when `-s` flag is used

## Testing

```bash
cargo test
```

## License

MIT License
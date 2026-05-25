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
```

### Example Output

```text
- unique1.txt
- nested\only1_nested.txt
+ nested\only2_nested.txt
+ unique2.txt
```

## Installation

```bash
cargo build --release
```

The executable will be in `target/release/super-compare.exe`

## Features

- Compares contents of two directories
- Shows files unique to each directory with `+` for dir2 and `-` for dir1
- Works with nested subdirectories
- Compares file sizes and shows size differences when `-s` flag is used

## Testing

```bash
cargo test
```

## License

MIT License
# Struct-Based Mini Stats Calculator

This binary (`using_struct.rs`) implements the same statistics calculator (Mean, Min, Max) but uses a Rust `struct` to encapsulate the data and behavior.

## Implementation Details

Instead of passing raw vectors to standalone functions, we define a `Dataset` struct:

```rust
struct Dataset {
    list: Vec<i32>
}
```

Methods are implemented on this struct to perform calculations:
- `Dataset::from_input()`: Factory method to create a dataset from user input.
- `mean()`: Calculates the average.
- `min()`: Finds the minimum value.
- `max()`: Finds the maximum value.

## Usage

To run this specific binary:

```bash
cargo run --bin using_struct
```

## Related Projects

- **Main Project**: [Mini Stats Calculator](https://github.com/lakshayknows/rust-mini-stats_calculator)
- **Learning Log**: [Rust Learning Log](https://github.com/lakshayknows/rust-learning-log)

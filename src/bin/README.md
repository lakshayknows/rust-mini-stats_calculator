# Struct-Based Mini Stats Calculator

This binary (`using_struct.rs`) implements the same statistics calculator (Mean, Min, Max) but uses Rust `structs` to encapsulate the data and behavior.

## Current Version (v2.0)

The current version introduces a nested struct `MetaData` to track the dataset's name and creation time.

### Key Features
- **Nested Structs**: `Dataset` now contains a `MetaData` struct.
- **Metadata Tracking**: Users can name their datasets, and the system automatically records the creation timestamp using `std::time::SystemTime`.
- **Method Chaining/Interaction**: The `main` loop now calls `dataset.data.describe()` before showing results.

```rust
struct MetaData {
    name: String,
    created_on: time::SystemTime
}

struct Dataset {
    list: Vec<i32>,
    data: MetaData
}
```

## Previous Versions

| Version | Description | Code Snippet |
| :--- | :--- | :--- |
| v1.0 | Initial struct implementation | `struct Dataset { list: Vec<i32> }` |

<details>
<summary>View v1.0 Code</summary>

```rust
struct Dataset {
    list: Vec<i32>
}

impl Dataset {
    fn max(&self) -> i32 { /* ... */ }
    fn min(&self) -> i32 { /* ... */ }
    fn mean(&self) -> f64 { /* ... */ }
    fn from_input() -> Dataset { /* ... */ }
}
```
</details>

## Usage

To run this specific binary:

```bash
cargo run --bin using_struct
```

## Related Projects

- **Main Project**: [Mini Stats Calculator](https://github.com/lakshayknows/rust-mini-stats_calculator)
- **Learning Log**: [Rust Learning Log](https://github.com/lakshayknows/rust-learning-log)

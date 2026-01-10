# Mini Stats Calculator

A simple command-line interface (CLI) tool written in Rust for calculating basic statistics from a list of numbers.

## Description

This project is a lightweight utility that allows users to input a list of comma-separated numbers and perform statistical operations. It is designed to be a quick and easy way to compute the mean, minimum, and maximum values of a dataset directly from the terminal.

## Features

-   **Calculate Mean**: Computes the average of the provided numbers.
-   **Find Minimum**: Identifies the smallest number in the list.
-   **Find Maximum**: Identifies the largest number in the list.
-   **Interactive Loop**: The program runs in a loop, allowing multiple calculations without restarting, until the user chooses to quit.

## Getting Started

### Prerequisites

-   Rust installed on your machine. If you don't have it, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/lakshayknows/rust-mini-stats_calculator.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd rust-mini-stats_calculator
    ```

### How to Run

Run the program using Cargo:

```bash
cargo run
```

## Usage

1.  Upon running the program, you will see a menu with the following options:
    ```text
    1. Mean
    2. Min
    3. Max
    4. Quit
    ```
2.  Enter the number corresponding to the operation you want to perform (e.g., `1` for Mean).
3.  When prompted, enter a list of integers separated by commas (e.g., `10, 20, 30, 40, 50`).
4.  The program will display the result and return to the main menu.
5.  Select `4` to exit the application.

### Example

**Calculating the Mean:**

```text
1.Mean
2.Min
3.Max
4.Quit
1
Input your list of numbers (comma separated):
10, 20, 30, 40, 50
size of the list: 5
Mean: 30
```


## Related Projects

This project is part of my Rust learning journey. You can find my learning log and other experiments in my [Rust Learning Log](https://github.com/lakshayknows/rust-learning-log) repository.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is open source and available under the [MIT License](LICENSE).

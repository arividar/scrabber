# Scrabber

## Overview

Scrabber is a command-line utility written in Rust for taking periodic screenshots. It's designed to be simple and efficient, with options for customizing the capture process.

## Features

- **Periodic Screenshots:** Captures screenshots at user-defined intervals.
- **Customizable Output:** Allows specifying the output directory for the screenshots.
- **Flexible Capture Options:**
    - Set the number of screenshots to take.
    - Run continuously until manually stopped (Ctrl-C).
    - Skip saving screenshots that are identical to the previous one to save space.
- **Organized Storage:** Automatically organizes screenshots into folders named by date (e.g., `YYYY-MM-DD`).
- **Cross-Platform:** Built with libraries that should support multiple operating systems.

## Dependencies

- `xcap`: For screen capturing.
- `image`: For image processing and saving.
- `clap`: For parsing command-line arguments.
- `chrono`: For handling timestamps.
- `log` and `env_logger`: For logging.
- `ctrlc`: For handling Ctrl-C interrupts.

## How to Run

1.  **Build the project:**
    ```bash
    cargo build
    ```
2.  **Run the application:**
    ```bash
    cargo run -- [OPTIONS]
    ```

### Examples

-   **Take a single screenshot and save it to the current directory:**
    ```bash
    cargo run
    ```
-   **Take 5 screenshots at 15-second intervals:**
    ```bash
    cargo run -- --count 5 --interval 15
    ```
-   **Take screenshots continuously and save them to a specific folder:**
    ```bash
    cargo run -- --forever --path /path/to/screenshots
    ```

## How to Run Tests

```bash
cargo test
```
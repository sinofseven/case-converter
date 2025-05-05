# Case Converter

Case Converter is a CLI tool built in Rust that transforms text between different case styles such as snake_case, camelCase, PascalCase, and more.

## Features

- Convert text to any of these formats:
  - snake_case
  - UPPER_SNAKE_CASE
  - camelCase
  - PascalCase
  - kebab-case
- Accept input from command-line arguments or stdin (pipes)
- Cross-platform compatibility (Windows, macOS, Linux)

## Installation

### From Source

1. Clone the repository
2. Build the project:
   ```bash
   cargo build --release
   ```
3. The binary will be available at `target/release/case-converter`

## Usage

### Command-line Arguments

```bash
# Convert to snake_case
$ case-converter -c snake_case "ExampleText"
example_text

# Convert to kebab-case
$ case-converter --case kebab-case "ExampleText"
example-text
```

### Piped Input

```bash
$ echo "ExampleText" | case-converter -c PascalCase
ExampleText
```

## Available Case Options

- `-c, --case <case>`: Specify the target case (required)
  - `snake_case` (aliases: `snake`, `Snake`)
  - `UPPER_SNAKE_CASE` (aliases: `upper_snake`, `upper-snake`, `UpperSnake`)
  - `camelCase` (aliases: `camel`, `Camel`)
  - `PascalCase` (aliases: `pascal`, `Pascal`) 
  - `kebab-case` (aliases: `kebab`)

## Help

For more information, use the help option:

```bash
$ case-converter --help
```
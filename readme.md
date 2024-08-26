# MDXTree

A Rust command-line tool that generates a file tree structure in MDX format for use with Astro's Starlight components.

https://starlight.astro.build/guides/components/#file-tree

## Features

- Generate a file tree structure from a specified directory
- Output to a file or stdout
- Option to include import statement for Starlight's FileTree component
- Respects .gitignore rules
- Handles hidden files and directories

## Installation

Install from crates.io:

```bash
cargo install mdxtree
```

## Usage

```bash
mdxtree [OPTIONS] [PATH]


Arguments:

- `PATH`: Path to generate file tree from (default: current directory)

Options:

- `-o, --output <FILE>`: Output file path (default: stdout)
- `-i, --include-import`: Include import statement for FileTree component
- `-h, --help`: Print help information
- `-V, --version`: Print version information
```

## Example

Generate a file tree for the current directory and output to stdout:

```base
mdxtree .
```

```markdown
<FileTree>
- Cargo.toml
- Cargo.lock
- readme.md
- .gitignore
- src/
  - main.rs
</FileTree>
```

## License

[MIT License](LICENSE)

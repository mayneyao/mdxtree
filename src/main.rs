use anyhow::Result;
use clap::Parser;
use ignore::WalkBuilder;
use serde_json::json; // Added import for json! macro
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to generate file tree from
    #[arg(default_value = ".")]
    path: String,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Include import statement
    #[arg(short, long)]
    include_import: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut writer: Box<dyn Write> = if let Some(output_path) = args.output {
        Box::new(BufWriter::new(File::create(output_path)?))
    } else {
        Box::new(BufWriter::new(std::io::stdout()))
    };

    generate_filetree_mdx(&args.path, &mut writer, args.include_import)?;

    Ok(())
}

fn generate_filetree_mdx<W: Write>(path: &str, writer: &mut W, include_import: bool) -> Result<()> {
    if include_import {
        writeln!(
            writer,
            "import {{ FileTree }} from '@astrojs/starlight/components';"
        )?;
        writeln!(writer)?;
    }
    writeln!(writer, "<FileTree>")?;

    let base_path = Path::new(path);
    let tree = generate_tree(base_path)?;
    write_tree(&tree, writer)?;

    writeln!(writer, "</FileTree>")?;
    Ok(())
}

fn generate_tree(base_path: &Path) -> Result<BTreeMap<String, Value>> {
    let mut tree = BTreeMap::new();

    let base_path_clone = base_path.to_path_buf();
    let walker = WalkBuilder::new(base_path)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .require_git(false)
        .filter_entry(move |entry| {
            entry.file_name() != ".git" && !entry.path().starts_with(base_path_clone.join(".git"))
        })
        .build();

    for entry in walker.filter_map(Result::ok) {
        let path = entry.path();
        if path == base_path {
            continue; // Skip the root directory itself
        }

        let relative = path.strip_prefix(base_path)?;
        let is_dir = path.is_dir();

        tree.insert(
            relative.to_string_lossy().into_owned(),
            json!({
                "type": if is_dir { "directory" } else { "file" }
            }),
        );
    }

    Ok(tree)
}

fn write_tree<W: Write>(tree: &BTreeMap<String, Value>, writer: &mut W) -> Result<()> {
    for (path, value) in tree {
        let components: Vec<_> = path.split('/').collect();
        let indent = "  ".repeat(components.len() - 1);
        let name = components.last().unwrap();
        let entry_type = value["type"].as_str().unwrap_or("unknown");

        if entry_type == "directory" {
            writeln!(writer, "{}- {}/", indent, name)?;
        } else {
            writeln!(writer, "{}- {}", indent, name)?;
        }
    }
    Ok(())
}

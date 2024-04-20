use anyhow::{Context, Result};

// command options and arguments
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, verbatim_doc_comment)]

pub struct CliArgs {
    /// File name to be parced
    drawio_file: String,
}

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    let text = std::fs::read_to_string(&cli_args.drawio_file).unwrap();

    let tree = roxmltree::Document::parse(&text)
        .with_context(|| format!("Failed parcing {} with roxmltree", &cli_args.drawio_file))?;
    // print!("{:?}", tree);

    for node in tree.descendants() {
        if node.is_element() {
            println!("node: {}", node.tag_name().name());
            for child in node.children() {
                if child.is_element() {
                    println!("    child: {}", child.tag_name().name());
                }
            }
        }
    }

    Ok(())
}

#[test]
fn verify_cli_args() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert()
}

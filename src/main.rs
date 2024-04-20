mod diagram_element;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use roxmltree::Node;
use std::collections::HashMap;

use crate::diagram_element::DiagramElement;

// command options and arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, verbatim_doc_comment)]
pub struct CliArgs {
    /// File to be parced
    drawio_file: String,
    #[arg(long)]
    /// Show debug information
    debug: bool,
}

// **************************************
fn read_diagram<'a>(diagram: Node<'a, 'a>) -> HashMap<&'a str, DiagramElement<'a>> {
    log::debug!("START diagram reading");
    let diagram_name = diagram.attribute("name").unwrap_or("n/a");
    log::debug!("diagram name: {}", diagram_name);
    let mut elements = HashMap::new();

    if let Some(diagram_root) = diagram.first_element_child() {
        if let Some(diagram_root) = diagram_root.first_element_child() {
            for raw_element in diagram_root.children() {
                if raw_element.is_element() {
                    match raw_element.tag_name().name() {
                        "mxCell" => {
                            let element = DiagramElement::read_mxcell(raw_element, diagram_name);
                            elements.insert(element.id, element);
                        }
                        "UserObject" => {
                            // println!("UserObject: {:?}", element.attributes())
                        }
                        &_ => {
                            log::debug!(
                                "unknown element: {} - skipping",
                                raw_element.tag_name().name()
                            );
                        }
                    }
                }
            }
        }
    }

    log::debug!("FINISH diagram reading");
    elements
}

// **************************************
fn main() -> Result<()> {
    let cli_args = CliArgs::parse();
    if cli_args.debug {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    log::debug!("START main");
    log::debug!("Arguments set by the user: {:?}", &cli_args);

    let text = std::fs::read_to_string(&cli_args.drawio_file).unwrap();

    let tree = roxmltree::Document::parse(&text)
        .with_context(|| format!("Failed parcing {} with roxmltree", &cli_args.drawio_file))?;

    let root_element = tree.root_element();
    if !root_element.has_tag_name("mxfile") {
        return Err(anyhow!("file is not drawio!"));
    }

    let drawio_host = root_element.attribute("host").unwrap_or("n/a");
    let drawio_version = root_element.attribute("version").unwrap_or("n/a");
    log::debug!("drawio host: {}, version: {}", drawio_host, drawio_version);

    for child in root_element.children() {
        if child.is_element() && child.has_tag_name("diagram") {
            // read one page (diagram)
            let elements = read_diagram(child);
            // export page elements
            for e in elements {
                let (e_key, e_val) = e;
                println!(
                    "  {}, value: {}, diagram_name: {}, link: {} -> {}",
                    e_key, e_val.value, e_val.diagram_name, e_val.source_id, e_val.target_id,
                );
            }
        }
    }
    log::debug!("FINISH main");
    Ok(())
}

#[test]
fn verify_cli_args() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert()
}

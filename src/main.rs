mod diagram_element;

use crate::diagram_element::ElementType;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use roxmltree::Node;

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
fn read_diagram<'a>(diagram: Node<'a, 'a>) -> Vec<DiagramElement<'a>> {
    log::debug!("START diagram reading");
    let page_name = diagram.attribute("name").unwrap_or("n/a");
    log::debug!("diagram page name: {}", page_name);

    let mut elements = Vec::new();

    let mut top_element_id = "";
    let mut sort: u32 = 0;
    let mut current_layer_n: u8 = 0;
    let mut current_layer = String::new();

    if let Some(diagram_root) = diagram.first_element_child() {
        if let Some(diagram_root) = diagram_root.first_element_child() {
            for raw_element in diagram_root.children() {
                if raw_element.is_element() {
                    if let Some(mut element) = DiagramElement::get(raw_element) {
                        element.sort = sort;
                        sort += 1;
                        element.diagram_page_name = page_name;
                        // save 1st - root element
                        if element.parent_id == tarpar::NO_VALUE {
                            top_element_id = element.id;
                            element.element_type = ElementType::Top;
                        }
                        // check if element is layer
                        if element.parent_id == top_element_id {
                            element.element_type = ElementType::Layer;
                            current_layer_n += 1;
                            if element.value == tarpar::NO_VALUE {
                                element.value = "background".to_string();
                            }
                            current_layer = format!("{:02} ({})", current_layer_n, element.value);
                        }
                        element.layer = current_layer.clone();
                        elements.push(element);
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

    // println!("{:?}", tree);

    let root_element = tree.root_element();
    if !root_element.has_tag_name("mxfile") {
        return Err(anyhow!("file is not drawio!"));
    }

    let drawio_host = root_element.attribute("host").unwrap_or(tarpar::NO_VALUE);
    let drawio_version = root_element
        .attribute("version")
        .unwrap_or(tarpar::NO_VALUE);

    log::debug!("drawio host: {}, version: {}", drawio_host, drawio_version);
    let mut diagram_page_n: u8 = 0;

    for child in root_element.children() {
        if child.is_element() && child.has_tag_name("diagram") {
            // read one page (diagram)
            diagram_page_n += 1;
            let mut elements: Vec<DiagramElement<'_>> = read_diagram(child);

            // process elements
            for e_val in elements.iter_mut() {
                e_val.diagram_page_n = diagram_page_n;
                e_val.drawio_host = drawio_host;
                e_val.drawio_version = drawio_version;
            }
            // export elements
            println!(
                "sort;\"type\";\"value\";\"action\";\"tags\";\"tooltip\";\"cluster\";\"jira\";\"color\";\"layer\";\"diagram\";\"drawio\";\"id\";\"parent_id\";"
            );
            for e_val in &elements {
                println!(
                    "{:02}{:04};\"{:?}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}-{}\";\"{}-{}\";\"{}\";\"{}\";",
                    e_val.diagram_page_n,
                    e_val.sort,
                    e_val.element_type,
                    e_val.value,
                    e_val.action,
                    e_val.tags,
                    e_val.tooltip,
                    e_val.cluster,
                    e_val.jira,
                    e_val.color,
                    e_val.layer,
                    e_val.diagram_page_n,
                    e_val.diagram_page_name,
                    e_val.drawio_host,
                    e_val.drawio_version,
                    e_val.id,
                    e_val.parent_id,
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

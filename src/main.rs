mod diagram_element;

use crate::diagram_element::DiagramElement;
use crate::diagram_element::{DiagramElementShort, ElementType};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use roxmltree::Node;
use std::collections::HashMap;

// command options and arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, verbatim_doc_comment, arg_required_else_help = true)]
pub struct CliArgs {
    /// File to be parsed
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

    let mut elements: Vec<DiagramElement<'_>> = Vec::new();

    let mut top_element_id = "";
    let mut sort: u32 = 0;
    let mut current_layer_n: u8 = 0;
    let mut current_layer = String::new();

    if let Some(diagram_root) = diagram.first_element_child() {
        if let Some(diagram_root) = diagram_root.first_element_child() {
            for raw_element in diagram_root.children() {
                if raw_element.is_element() {
                    if let Some(mut element) = DiagramElement::get(raw_element) {
                        // save 1st - ROOT element
                        if element.parent_id == tarpar::NO_VALUE {
                            top_element_id = element.id;
                            element.element_type = ElementType::Top;
                        }
                        // SORT
                        element.sort = sort;
                        sort += 1;
                        // DIAGRAM_NAME
                        element.diagram_page_name = page_name;
                        // LAYER
                        if element.parent_id == top_element_id {
                            element.element_type = ElementType::Layer;
                            current_layer_n += 1;
                            if element.value == "" {
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
fn normalize_string(in_str: String) -> String {
    in_str
        .replace("\n", " ")
        .replace("\"", "'")
        .replace(";", ",")
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

    let text = std::fs::read_to_string(&cli_args.drawio_file)
        .with_context(|| format!("Failed reading file {}", &cli_args.drawio_file))?;

    let tree = roxmltree::Document::parse(&text)
        .with_context(|| format!("Failed parsing {} with roxmltree", &cli_args.drawio_file))?;

    let root_element = tree.root_element();
    if !root_element.has_tag_name("mxfile") {
        return Err(anyhow!("File is not drawio!"));
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
            let mut indexed_elements: HashMap<&str, DiagramElementShort> = Default::default();

            // post processing elements
            // set common fields
            for e_val in elements.iter_mut() {
                e_val.diagram_page_n = diagram_page_n;
                e_val.drawio_host = drawio_host;
                e_val.drawio_version = drawio_version;
            }
            // process users and systems
            let mut current_system_id = "";
            let mut current_object = "".to_string();
            let mut current_object_type = "".to_string();
            let mut current_team = "";
            for e_val in elements.iter_mut() {
                // process user
                if e_val.element_type == ElementType::Shape("umlActor".to_string()) {
                    e_val.object = format!("{}", e_val.value);
                    e_val.object_type = format!("Пользователь");
                    indexed_elements.insert(
                        e_val.id,
                        DiagramElementShort {
                            object: e_val.object.clone(),
                            team: e_val.team,
                        },
                    );
                    continue;
                };
                // process system
                if e_val.element_type == ElementType::System {
                    current_system_id = e_val.id;
                    current_object = format!("{}", e_val.value);
                    e_val.object = current_object.clone();
                    current_object_type = format!("Система");
                    e_val.object_type = current_object_type.clone();
                    e_val.set_action("line"); //for system we use color of border line
                    current_team = e_val.team;
                    indexed_elements.insert(
                        e_val.id,
                        DiagramElementShort {
                            object: e_val.object.clone(),
                            team: e_val.team,
                        },
                    );
                    continue;
                };
                // process system function
                if (e_val.element_type == ElementType::TextBlock)
                    & (e_val.parent_id == current_system_id)
                {
                    e_val.object = current_object.clone();
                    e_val.object_type = current_object_type.clone();
                    e_val.element_type = ElementType::SystemFunction;
                    e_val.set_action("text"); //for system function we use text color

                    // inherit team from system
                    if e_val.team == "" {
                        e_val.team = current_team
                    }
                    indexed_elements.insert(
                        e_val.id,
                        DiagramElementShort {
                            object: e_val.object.clone(),
                            team: e_val.team,
                        },
                    );
                }
            }

            // process integrations
            let mut current_link_id = "";
            let mut current_action = "";
            let mut current_source_team = "";
            let mut current_object = "".to_string();
            let mut current_object_type = "".to_string();
            for e_val in elements.iter_mut() {
                if e_val.element_type == ElementType::Link {
                    current_link_id = e_val.id;
                    let source_object: String;
                    (source_object, current_source_team) =
                        if let Some(e) = indexed_elements.get(e_val.source_id) {
                            (e.object.clone(), e.team)
                        } else {
                            ("___".to_string(), "")
                        };
                    let target_object = if let Some(e) = indexed_elements.get(e_val.target_id) {
                        e.object.clone()
                    } else {
                        "___".to_string()
                    };
                    current_object = format!("{} <--> {}", source_object, target_object);
                    e_val.object = current_object.clone();
                    current_object_type = format!("Интеграция");
                    e_val.object_type = current_object_type.clone();
                    e_val.set_action("line"); //for link we use color of link line
                    current_action = e_val.action;
                    // check if label color differs from line
                    if (e_val.color_text == tarpar::COLOR_BLUE)
                        | (e_val.color_text == tarpar::COLOR_GREEN)
                        | (e_val.color_text == tarpar::COLOR_RED)
                    {
                        e_val.set_action("text");
                    }
                    // inherit team from source system\system function
                    if e_val.team == "" {
                        e_val.team = current_source_team
                    }
                    continue;
                };
                if (e_val.element_type == ElementType::LinkLabel)
                    & (e_val.parent_id == current_link_id)
                {
                    e_val.object = current_object.clone();
                    e_val.object_type = current_object_type.clone();
                    // find out correct action
                    if (e_val.color_text == tarpar::COLOR_BLUE)
                        | (e_val.color_text == tarpar::COLOR_GREEN)
                        | (e_val.color_text == tarpar::COLOR_RED)
                    {
                        e_val.set_action("text");
                    } else {
                        e_val.action = current_action // for link labels we take action from link by default
                    }
                    // inherit team from source system\system function
                    if e_val.team == "" {
                        e_val.team = current_source_team
                    }
                }
            }

            // export elements
            println!(
                "sort;\"object type\";\"object\";\"value\";\"action\";\"tags\";\"tooltip\";\"team\";\"tasks\";\"type\";\"color text\";\"color line\";\"layer\";\"diagram\";\"drawio\";\"id\";\"parent_id\";"
            );
            for e_val in &elements {
                println!(
                    "{:02}{:04};\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}-{}\";\"{}-{}\";\"{}\";\"{}\";",
                    e_val.diagram_page_n,
                    e_val.sort,
                    e_val.object_type,
                    normalize_string(e_val.object.clone()),
                    normalize_string(e_val.value.clone()),
                    e_val.action,
                    normalize_string(e_val.tags.to_string()),
                    normalize_string(e_val.tooltip.to_string()),
                    normalize_string(e_val.team.to_string()),
                    normalize_string(e_val.tasks.to_string()),
                    normalize_string(format!("{:?}", e_val.element_type)),
                    e_val.color_text,
                    e_val.color_line,
                    normalize_string(e_val.layer.clone()),
                    e_val.diagram_page_n,
                    normalize_string(e_val.diagram_page_name.to_string()),
                    normalize_string(e_val.drawio_host.to_string()),
                    normalize_string(e_val.drawio_version.to_string()),
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

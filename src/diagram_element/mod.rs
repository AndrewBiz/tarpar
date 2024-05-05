use regex::Regex;
use roxmltree::Node;

// **************************************
#[derive(Debug, PartialEq)]
pub enum ElementType {
    Top,
    Layer,
    Area,
    Group,
    TextBlock,
    System,
    SystemFunction,
    Link,
    LinkLabel,
    Shape(String),
    None,
}

// **************************************
#[derive(Debug)]
pub struct DiagramElement<'a> {
    pub object_type: String,
    pub object: String,
    pub element_type: ElementType,
    pub sort: u32,
    pub id: &'a str,
    pub parent_id: &'a str,
    pub value: String,
    pub color_text: String,
    pub color_line: String,
    pub action: &'a str,
    pub tags: &'a str,
    pub tooltip: &'a str,
    pub cluster: &'a str,
    pub jira: &'a str,
    pub source_id: &'a str,
    pub target_id: &'a str,
    pub diagram_page_n: u8,
    pub diagram_page_name: &'a str,
    pub layer: String,
    pub drawio_host: &'a str,
    pub drawio_version: &'a str,
}
// **************************************
#[derive(Debug)]
pub struct DiagramElementShort {
    pub object: String,
}

// **************************************
fn get_element_type(style: &str) -> ElementType {
    if style == tarpar::NO_VALUE {
        ElementType::None
    } else if style.contains("group") {
        ElementType::Group
    } else if style.contains("text;") {
        ElementType::TextBlock
    } else if style.contains("swimlane") {
        ElementType::System
    } else if style.contains("edgeStyle=") {
        ElementType::Link
    } else if style.contains("edgeLabel") {
        ElementType::LinkLabel
    } else if style.contains("shape=") {
        let re = Regex::new(r"shape=(?<shape_name>[A-Za-z0-9._]+);").unwrap();
        if let Some(caps) = re.captures(&style) {
            ElementType::Shape(caps["shape_name"].to_string())
        } else {
            ElementType::Shape("_".to_string())
        }
    } else {
        ElementType::Area
    }
}

// **************************************
fn get_text_color(raw_style: &str, raw_value: &str) -> String {
    // try read font color from html (font tag)
    let html_fragment = scraper::Html::parse_fragment(raw_value);
    let html_selector = scraper::Selector::parse(r#"font"#).unwrap();
    let text_color1 = if let Some(html_node) = html_fragment.select(&html_selector).next() {
        if let Some(font_color) = html_node.value().attr("color") {
            Some(font_color.to_string())
        } else {
            None
        }
    } else {
        None
    };
    // TODO try read font color from html (rgb tag)
    // example "<span style=\"border-color: var(--border-color); caret-color: rgb(0, 153, 0); color: rgb(0, 153, 0); font-size: 13px;\">- Динамика ТП&nbsp;</span>"
    let text_color2: Option<String> = None;

    // try read font color from style (fontColor tag)
    let re = Regex::new(r"fontColor=(?<color>[A-Za-z0-9#]{7})").unwrap();
    let text_color3: Option<String> = if let Some(caps) = re.captures(&raw_style) {
        Some(caps["color"].to_string())
    } else {
        None
    };

    // get color
    let color = if let Some(color1) = text_color1 {
        color1.to_uppercase()
    } else if let Some(color2) = text_color2 {
        color2.to_uppercase()
    } else if let Some(color3) = text_color3 {
        color3.to_uppercase()
    } else {
        "default".to_string()
    };
    color
}

// **************************************
fn get_line_color(raw_style: &str) -> String {
    // try read line color from style (strokeColor tag)
    let re = Regex::new(r"strokeColor=(?<color>[A-Za-z0-9#]{7})").unwrap();
    if let Some(caps) = re.captures(&raw_style) {
        caps["color"].to_string().to_uppercase()
    } else {
        "default".to_string()
    }
}

// **************************************
impl<'a> DiagramElement<'a> {
    pub fn get(raw_element: Node<'a, 'a>) -> Option<DiagramElement<'a>> {
        let raw_element_name = raw_element.tag_name().name();
        log::debug!("START diagram element processing {}", raw_element_name);

        let result = match raw_element_name {
            "mxCell" | "UserObject" => {
                // Reading raw tag values
                // ID
                let id = raw_element.attribute("id").unwrap_or(tarpar::NO_VALUE);
                log::debug!("ID: {}", id,);

                // VALUE (LABEL)
                let raw_value = if let Some(value) = raw_element.attribute("value") {
                    value
                } else if let Some(label) = raw_element.attribute("label") {
                    label
                } else {
                    ""
                };

                // TAGS, TOOLTIP, CLUSTER, JIRA
                let (tags, tooltip, cluster, jira) = if raw_element_name == "UserObject" {
                    // TAGS
                    let tags = raw_element.attribute("tags").unwrap_or("");
                    // TOOLTIP
                    let tooltip = raw_element.attribute("tooltip").unwrap_or("");
                    // CLUSTER
                    let cluster = raw_element.attribute("cluster").unwrap_or("");
                    // JIRA
                    let jira = raw_element.attribute("jira").unwrap_or("");
                    (tags, tooltip, cluster, jira)
                } else {
                    ("", "", "", "")
                };

                // STYLE, PARENT, SOURCE, TARGET
                // getting correct mxCell
                let raw_element = if raw_element_name == "UserObject" {
                    match raw_element.first_element_child() {
                        Some(child_element) => child_element,
                        None => raw_element,
                    }
                } else {
                    raw_element
                };

                // STYLE
                let raw_style = raw_element.attribute("style").unwrap_or(tarpar::NO_VALUE);
                // PARENT
                let parent_id = raw_element.attribute("parent").unwrap_or(tarpar::NO_VALUE);
                // SOURCE
                let source_id = raw_element.attribute("source").unwrap_or(tarpar::NO_VALUE);
                // TARGET
                let target_id = raw_element.attribute("target").unwrap_or(tarpar::NO_VALUE);
                // COLOR_TEXT
                let color_text = get_text_color(raw_style, raw_value);
                // COLOR_LINE
                let color_line = get_line_color(raw_style);

                // TODO postprocessing
                // Checking out the type of element
                let element_type = get_element_type(raw_style);

                // removing html stuff from text - get pure text for value
                let fragment = scraper::Html::parse_fragment(raw_value);
                let html_selector = scraper::Selector::parse(r#"html"#).unwrap();
                let html_node = fragment.select(&html_selector).next().unwrap();
                let text_vec: Vec<&str> = html_node
                    .text()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.trim())
                    .collect();
                let value = text_vec.join(" ");

                // rest of fields (will be set later)
                let object_type = "".to_string();
                let object = "".to_string();
                let sort = 0;
                let diagram_page_n: u8 = 0;
                let diagram_page_name = "";
                let layer = "".to_string();
                let action = "";
                let drawio_host = "";
                let drawio_version = "";

                Some(Self {
                    object_type,
                    object,
                    element_type,
                    sort,
                    id,
                    parent_id,
                    value,
                    color_text,
                    color_line,
                    action,
                    tags,
                    tooltip,
                    cluster,
                    jira,
                    source_id,
                    target_id,
                    diagram_page_n,
                    diagram_page_name,
                    layer,
                    drawio_host,
                    drawio_version,
                })
            }
            &_ => {
                log::debug!("unknown drawio element - skipping",);
                None
            }
        };
        log::debug!("FINISH diagram element processing");
        result
    }
}

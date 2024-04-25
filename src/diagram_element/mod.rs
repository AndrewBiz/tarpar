use regex::Regex;
use roxmltree::Node;

// **************************************
#[derive(Debug)]
pub enum ElementType {
    Top,
    Layer,
    Area,
    Group,
    TextBlock,
    System,
    // SystemFunction,
    Link,
    LinkLabel,
    Shape(String),
    None,
}

// **************************************
#[derive(Debug)]
pub struct DiagramElement<'a> {
    pub element_type: ElementType,
    pub sort: u32,
    pub id: &'a str,
    pub parent_id: &'a str,
    pub value: String,
    pub text_color: String,
    // color_style: &'a str,
    // color_text: &'a str,
    // label: &'a str,
    // tags: &'a str,
    // tooltip: &'a str,
    pub source_id: &'a str,
    pub target_id: &'a str,
    pub diagram_page_n: u8,
    pub diagram_page_name: &'a str,
    pub layer: String,
    pub drawio_host: &'a str,
    pub drawio_version: &'a str,
}

// **************************************
fn get_element_type(style: &str) -> ElementType {
    if style.contains("group") {
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
impl<'a> DiagramElement<'a> {
    pub fn read_mxcell(raw_element: Node<'a, 'a>) -> DiagramElement<'a> {
        log::debug!("START diagram element processing");

        let id = raw_element.attribute("id").unwrap_or(tarpar::NO_VALUE);
        log::debug!("tag_name: {}, ID: {}", raw_element.tag_name().name(), id,);

        let parent_id = raw_element.attribute("parent").unwrap_or(tarpar::NO_VALUE);

        // Checking out the type of element
        let (style, element_type) = if let Some(style) = raw_element.attribute("style") {
            (style, get_element_type(style))
        } else {
            (tarpar::NO_VALUE, ElementType::None)
        };

        // reading text value (try value then label)
        let raw_value = if let Some(value) = raw_element.attribute("value") {
            value
        } else if let Some(label) = raw_element.attribute("label") {
            label
        } else {
            tarpar::NO_VALUE
        };
        // TODO reading text color
        // try read font color from html
        let fragment = scraper::Html::parse_fragment(raw_value);
        let html_selector = scraper::Selector::parse(r#"font"#).unwrap();
        let raw_color = if let Some(html_node) = fragment.select(&html_selector).next() {
            if let Some(font_color) = html_node.value().attr("color") {
                Some(font_color)
            } else {
                None
            }
        } else {
            None
        };
        // try read font color from style
        let text_color = if let Some(color) = raw_color {
            color.to_string()
        } else {
            let re = Regex::new(r"fontColor=(?<color>[A-Za-z0-9#]{7})").unwrap();
            if let Some(caps) = re.captures(&style) {
                caps["color"].to_string()
            } else {
                "no_value".to_string()
            }
        };
        // removing html stuff from text
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

        let source_id = raw_element.attribute("source").unwrap_or(tarpar::NO_VALUE);
        let target_id = raw_element.attribute("target").unwrap_or(tarpar::NO_VALUE);
        let sort = 0;
        let diagram_page_n: u8 = 0;
        let diagram_page_name = "";
        let layer = "".to_string();
        let drawio_host = "";
        let drawio_version = "";

        log::debug!("FINISH diagram element processing");
        Self {
            element_type,
            sort,
            id,
            parent_id,
            value,
            text_color,
            source_id,
            target_id,
            diagram_page_n,
            diagram_page_name,
            layer,
            drawio_host,
            drawio_version,
        }
    }
}

use roxmltree::Node;
use scraper::element_ref;
// use scraper::Html;

// **************************************
#[derive(Debug)]
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
    pub id: &'a str,
    pub parent_id: &'a str,
    pub value: &'a str,
    pub element_type: ElementType,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    // color_style: &'a str,
    // color_text: &'a str,
    // label: &'a str,
    // tags: &'a str,
    // tooltip: &'a str,
    pub source_id: &'a str,
    pub target_id: &'a str,
    pub diagram_page_n: u8,
    pub diagram_page_name: &'a str,
    pub drawio_host: &'a str,
    pub drawio_version: &'a str,
}

// **************************************
fn get_element_type(style: &str) -> ElementType {
    if style.contains("group;") {
        ElementType::Group
    } else if style.contains("text;") {
        ElementType::TextBlock
    } else if style.contains("swimlane;") {
        ElementType::System
    } else if style.contains("edgeStyle=") {
        ElementType::Link
    } else if style.contains("edgeLabel;") {
        ElementType::LinkLabel
    } else if style.contains("shape=") {
        ElementType::Shape("TODO".to_string())
    } else {
        ElementType::Area
    }
}

// **************************************
impl<'a> DiagramElement<'a> {
    pub fn read_mxcell(raw_element: Node<'a, 'a>) -> DiagramElement<'a> {
        log::debug!("START diagram element processing");
        log::debug!(
            "diagram element tag_name: {}",
            raw_element.tag_name().name()
        );

        let id = raw_element.attribute("id").unwrap_or(tarpar::NO_VALUE);
        let parent_id = raw_element.attribute("parent").unwrap_or(tarpar::NO_VALUE);

        // Checking out the type of element
        let style;
        let element_type;
        match raw_element.attribute("style") {
            Some(style) => element_type = get_element_type(style),
            None => {
                style = tarpar::NO_VALUE;
                element_type = ElementType::None
            }
        }

        // TODO html off
        let value = raw_element.attribute("value").unwrap_or(tarpar::NO_VALUE);

        let color_r = 0;
        let color_g = 0;
        let color_b = 0;
        let source_id = raw_element.attribute("source").unwrap_or(tarpar::NO_VALUE);
        let target_id = raw_element.attribute("target").unwrap_or(tarpar::NO_VALUE);

        let diagram_page_n: u8 = 0;
        let diagram_page_name = "";
        let drawio_host = "";
        let drawio_version = "";

        log::debug!("FINISH diagram element processing");
        Self {
            id,
            parent_id,
            value,
            element_type,
            color_r,
            color_g,
            color_b,
            source_id,
            target_id,
            diagram_page_n,
            diagram_page_name,
            drawio_host,
            drawio_version,
        }
    }
}

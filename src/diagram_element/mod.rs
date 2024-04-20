use std::io::StdoutLock;

use roxmltree::Node;

// **************************************
#[derive(Debug)]
pub struct DiagramElement<'a> {
    pub id: &'a str,
    pub parent_id: &'a str,
    pub value: &'a str,
    // obj_type: String,
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
    pub diagram_name: &'a str,
}

// **************************************
impl<'a> DiagramElement<'a> {
    pub fn read_mxcell(raw_element: Node<'a, 'a>, diagram_name: &'a str) -> DiagramElement<'a> {
        log::debug!("START diagram element processing");
        log::debug!(
            "diagram element tag_name: {}",
            raw_element.tag_name().name()
        );

        let id = raw_element.attribute("id").unwrap_or("not_found");
        let parent_id = raw_element.attribute("parent").unwrap_or("not_found");
        let value = raw_element.attribute("value").unwrap_or("not_found");

        let color_r = 0;
        let color_g = 0;
        let color_b = 0;
        let source_id = raw_element.attribute("source").unwrap_or("not_found");
        let target_id = raw_element.attribute("target").unwrap_or("not_found");

        log::debug!("FINISH diagram element processing");
        Self {
            id,
            parent_id,
            value,
            color_r,
            color_g,
            color_b,
            source_id,
            target_id,
            diagram_name,
        }
    }
}

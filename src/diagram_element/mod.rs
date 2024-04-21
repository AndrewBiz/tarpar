use roxmltree::Node;
use scraper::Html;

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
    pub diagram_page_n: u8,
    pub diagram_page_name: &'a str,
    pub drawio_host: &'a str,
    pub drawio_version: &'a str,
}

// **************************************
impl<'a> DiagramElement<'a> {
    pub fn read_mxcell(raw_element: Node<'a, 'a>) -> DiagramElement<'a> {
        log::debug!("START diagram element processing");
        log::debug!(
            "diagram element tag_name: {}",
            raw_element.tag_name().name()
        );

        let id = raw_element.attribute("id").unwrap_or("not_found");
        let parent_id = raw_element.attribute("parent").unwrap_or("not_found");
        let value = raw_element.attribute("value").unwrap_or("not_found");

        let style = raw_element.attribute("style").unwrap_or("");
        println!("style='{}'", style);

        let html_fragment = Html::parse_fragment(value);
        // println!("!html!!! {:?}", html_fragment);

        let color_r = 0;
        let color_g = 0;
        let color_b = 0;
        let source_id = raw_element.attribute("source").unwrap_or("not_found");
        let target_id = raw_element.attribute("target").unwrap_or("not_found");

        let diagram_page_n: u8 = 0;
        let diagram_page_name = "";
        let drawio_host = "";
        let drawio_version = "";

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
            diagram_page_n,
            diagram_page_name,
            drawio_host,
            drawio_version,
        }
    }
}

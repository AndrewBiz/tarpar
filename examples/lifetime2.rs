use tarpar;
const OBJECT_TYPE: &str = "xyz";

// **************************************
#[derive(Debug)]
pub struct Element<'a> {
    pub id: String,
    pub color_text: String,
    pub color_line: String,
    pub action: &'a str,
    pub object_type: &'a str,
}

fn get_action(color: String) -> &'static str {
    match color.as_str() {
        tarpar::COLOR_BLACK | "default" => tarpar::ACTION_USE,
        tarpar::COLOR_GREEN => tarpar::ACTION_CREATE,
        tarpar::COLOR_BLUE => tarpar::ACTION_MODIFY,
        tarpar::COLOR_RED => tarpar::ACTION_REMOVE,
        _ => tarpar::ACTION_ERROR,
    }
}
fn main() {
    // println!("Usage:\n\tcargo run --example lifetime1");

    // generate collection
    let mut elements: Vec<Element<'_>> = Vec::new();
    for i in 1..11 {
        let e = Element {
            id: i.to_string(),
            color_text: tarpar::COLOR_BLUE.to_string(),
            color_line: "#009900".to_string(),
            action: "",
            object_type: "",
        };
        elements.push(e);
    }
    // postprocess1
    for e in elements.iter_mut() {
        e.object_type = OBJECT_TYPE;
    }
    // postprocess2
    for e in elements.iter_mut() {
        e.action = get_action(e.color_text.clone());
    }
    // print1
    for e in &elements {
        println!("{:?}", e);
    }
    // postprocess2
    for e in elements.iter_mut() {
        e.action = get_action(e.color_line.clone());
    }
    // print2
    for e in &elements {
        println!("{:?}", e);
    }
}

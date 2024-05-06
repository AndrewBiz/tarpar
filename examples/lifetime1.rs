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

fn main() {
    // println!("Usage:\n\tcargo run --example lifetime1");

    // generate collection
    let mut elements: Vec<Element<'_>> = Vec::new();
    for i in 1..11 {
        let e = Element {
            id: i.to_string(),
            color_text: "#009900".to_string(),
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
    // print
    for e in &elements {
        println!("{:?}", e);
    }
}

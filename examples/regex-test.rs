use regex::Regex;
fn main() {
    let raw_value = r#"<span style="border-color: var(--border-color); caret-color: rgb(33, 163, 44); color: rgb(0, 127, 255); font-size: 13px;">- Динамика ТП&nbsp;</span>"#;

    let re = Regex::new(
        r"[ ;]{1}color: rgb[(]{1}(?<red>\d{0,3}), (?<green>\d{1,3}), (?<blue>\d{0,3})[)]{1}",
    )
    .unwrap();

    let text_color2: Option<String> = if let Some(caps) = re.captures(&raw_value) {
        let rgb = caps["red"].parse::<u16>().unwrap_or(0) * 256 * 256
            + caps["green"].parse::<u16>().unwrap_or(0) * 256
            + caps["blue"].parse::<u16>().unwrap_or(0);

        let rgb_str = format!("{:#08x}", rgb).to_uppercase();
        // rgb_str.replace_range(0..2, "#");
        Some(format!("#{}", &rgb_str[2..]))
    } else {
        None
    };
    println!("{:#?}", text_color2);
}

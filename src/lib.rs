pub const NO_VALUE: &str = "no_value";

pub const COLOR_BLACK: &str = "#000000";
pub const COLOR_GREEN: &str = "#009900";
pub const COLOR_BLUE: &str = "#007FFF";
pub const COLOR_RED: &str = "#FF0000";

pub const ACTION_USE: &str = "";
pub const ACTION_CREATE: &str = "Создать";
pub const ACTION_MODIFY: &str = "Доработать";
pub const ACTION_REMOVE: &str = "Вывести из эксплуатации";
pub const ACTION_ERROR: &str = "_неизвестное действие_";

pub fn get_action(color: &String) -> &str {
    match color.as_str() {
        COLOR_BLACK | "default" => ACTION_USE,
        COLOR_GREEN => ACTION_CREATE,
        COLOR_BLUE => ACTION_MODIFY,
        COLOR_RED => ACTION_REMOVE,
        _ => ACTION_ERROR,
    }
}

use inkeeper_domain::ProgramName;
use regex::Regex;
use std::collections::HashMap;

pub fn get_program_name_to_string_mapping() -> HashMap<ProgramName, &'static str> {
    HashMap::from([
        (ProgramName::Photoshop, "photoshop"),
        (ProgramName::Sai, "sai"),
    ])
}

pub fn get_tasklist_row_parse_regex() -> Regex {
    Regex::new(r"^(.+)\.exe.+\d:\d\d:\d\d\s(.+)$").unwrap()
}

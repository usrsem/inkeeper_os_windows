use inkeeper_domain::{ProgramInfo, ProgramName};
use inkeeper_os_boundary::boundary::Os;
use std::ffi::CString;
use std::process::Command;
use std::str;

mod config;

pub struct WindowsOs {}

impl Os for WindowsOs {
    fn get_program_info(&self, program_name: ProgramName) -> Option<ProgramInfo> {
        let mapping = config::get_program_name_to_string_mapping();
        let program_name = mapping.get(&program_name).unwrap();
        let program_info = find_program_info(program_name);

        match program_info {
            Some(info) => Some(ProgramInfo {
                is_minimized: is_minimized(&info.window_title),
                ..info
            }),
            None => None,
        }
    }
}

fn find_program_info(program_name: &str) -> Option<ProgramInfo> {
    let programs = exec_tasklist();
    let programs_info = parse_tasklist_rows(&programs);
    programs_info
        .iter()
        .find(|info| info.program_name == program_name)
        .map(|info| info.clone())
}

fn exec_tasklist() -> String {
    let buf = Command::new("tasklist").arg("\\v").output().unwrap().stdout;

    match str::from_utf8(&buf) {
        Result::Ok(s) => String::from(s),
        _ => String::new(),
    }
}

fn parse_tasklist_rows(rows: &str) -> Vec<ProgramInfo> {
    config::get_tasklist_row_parse_regex()
        .captures_iter(rows)
        .map(|cap| ProgramInfo::new(&cap[1].to_string(), &cap[2].to_string()))
        .collect()
}

fn is_minimized(program_title: &str) -> bool {
    unsafe {
        let window_name = CString::new(program_title).unwrap();
        let window_handle = user32::FindWindowA(std::ptr::null_mut(), window_name.as_ptr());
        let is_minimized = user32::IsIconic(window_handle);
        is_minimized != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execs_tasklist() {
        let tasks = exec_tasklist();
        assert!(tasks.len() > 10);
    }

    #[test]
    fn parses_one_row_from_tasklist() {}

    #[test]
    fn parses_many_rows_from_tasklist() {}
}

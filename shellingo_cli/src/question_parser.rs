use std::{collections::HashSet, fs::{self}, path::PathBuf};
use shellingo_core::question::Question;
use walkdir:: {WalkDir, DirEntry};

/// Returns all Questions under the provided path. 
/// Takes both a single file or a directory and recursively parses all Questions under them.
pub fn read_all_questions_from(path: PathBuf) -> HashSet<Question> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(filter_for_files)
            .filter_map(read_file_to_string_or_skip_on_error)
            .flat_map(get_lines_from_string)
            .map(parse_question_from_line)
            .collect()
}

fn filter_for_files(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_file()
}

fn read_file_to_string_or_skip_on_error(file_entry: DirEntry) -> Option<String> {
    match fs::read_to_string(file_entry.path()) {
        Ok(file_str) => return Some(file_str),
        Err(_) => {
            println!("Error: Skipping unreadable file: {}", file_entry.path().display());
            return None;
        }
    }
}

fn get_lines_from_string(file_str: String) -> std::vec::IntoIter<String> {
    file_str.lines()
        .map(str::to_owned)
        .collect::<Vec<String>>()
        .into_iter()
}

fn parse_question_from_line(line: String) -> Question {
   todo!() 
}

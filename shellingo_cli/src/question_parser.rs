use std::{collections::HashSet, fs, path::PathBuf};
use shellingo_core::question::Question;
use walkdir::WalkDir;

pub fn read_all_questions_from(path: PathBuf) -> HashSet<Question> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|dir_entry| dir_entry.path().is_file())
            .filter_map(|file_entry| {
                match fs::read_to_string(file_entry.path()) {
                    Ok(file_str) => return Some(file_str),
                    Err(_) => {
                        println!("Cannor read file: {}", file_entry.path().display());
                        return None;
                    }
                }
            })
            .flat_map(get_lines_from_file)
            .map(parse_question_from_line)
            .collect()
}

fn get_lines_from_file(file_str: String) -> std::vec::IntoIter<String> {
    file_str.lines()
        .map(str::to_owned)
        .collect::<Vec<String>>()
        .into_iter()
}

fn parse_question_from_line(line: String) -> Question {
   todo!() 
}

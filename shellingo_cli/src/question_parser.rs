use std::{collections::HashSet, fs::{self}, path::PathBuf, sync::LazyLock, vec::IntoIter};
use regex::Regex;
use shellingo_core::question::Question;
use walkdir::{WalkDir, DirEntry};

static MULTIPLE_WHITESPACES_RE: LazyLock<Regex> = LazyLock::new(||Regex::new(r"\s+").unwrap());


/// Returns all Questions under the provided path. 
/// Takes both a single file or a directory and recursively parses all Questions under them.
pub fn read_all_questions_from(path: PathBuf) -> HashSet<Question> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(filter_for_files)
            .filter_map(read_file_to_string_or_skip_on_error)
            .flat_map(get_lines_from_string)
            .filter_map(parse_question_from_line)
            .collect()
}

struct ProcessingStep<T> {
    result: T,
    path: String
}

fn filter_for_files(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_file()
}

fn read_file_to_string_or_skip_on_error(file: DirEntry) -> Option<ProcessingStep<String>> {
    let path = file.path();
    match fs::read_to_string(path) {
        Ok(file_str) => 
            return Some(ProcessingStep { 
                path: path.display().to_string(), 
                result: file_str 
            }),
        Err(_) => {
            println!("Error: Skipping unreadable file: {}", path.display());
            return None;
        }
    }
}

fn get_lines_from_string(contents: ProcessingStep<String>) -> Vec<ProcessingStep<String>> {
    let file_str = contents.result;
    file_str.lines()
        .map(str::to_owned)
        .map(|line| ProcessingStep::<String> {
            result: line, 
            path: contents.path.to_owned()
        })
        .collect()
}

fn parse_question_from_line(line_contents: ProcessingStep<String>) -> Option<Question> {
    let line = line_contents.result;
    let path = line_contents.path;
    if line.starts_with("#") { 
        return None; // Skip comments. 
    }; 
    let split: Vec<&str> = line.split_terminator('|').collect();
    if split.len() != 2 {
        print!("Error, skipping malformed question. Path:'{}' Line: '{}'", path, line);
        return None;
    }
    let question: String = remove_exra_whitespaces(split[0]);
    let solutions: HashSet<String> = HashSet::from([remove_exra_whitespaces(split[1])]);
    
    return Some(Question::new(path, question, solutions));
}

fn remove_exra_whitespaces(text: &str) -> String {
    MULTIPLE_WHITESPACES_RE.replace_all(text, "").into_owned()
}
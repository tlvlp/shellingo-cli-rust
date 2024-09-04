use std::{collections::{HashMap, HashSet}, fs::{self}, path::PathBuf, sync::LazyLock};
use regex::Regex;
use shellingo_core::question::Question;
use walkdir::{DirEntry, Error, WalkDir};

static MULTIPLE_WHITESPACES_REGEX: LazyLock<Regex> = LazyLock::new(||Regex::new(r"\s+").unwrap());


/// Returns all Questions under the provided path. 
/// Takes both a single file or a directory and recursively parses all Questions under them.
pub fn read_all_questions_from(path: PathBuf) -> HashMap<String, Question> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(filter_readable_entries)
            .filter(filter_for_files)
            .filter_map(read_file_to_string_or_skip_on_error)
            .flat_map(get_lines_from_string)
            .filter_map(parse_question_from_line)
            .fold(HashMap::new(), |map, new_question| merge_answers(map, new_question))
}

struct ProcessingStep<T> {
    result: T,
    path: String
}

fn filter_readable_entries(result: Result<DirEntry, Error>) -> Option<DirEntry> {
    match result {
        Ok(res) => return Some(res), 
        Err(e) => {
            print!("Error: Skipping unreadable directory entry with reason: {}", e);
            return None;
        },
    }
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
    let split_q: Vec<&str> = line.split_terminator('|').collect();
    if split_q.len() != 2 {
        print!("Error, skipping malformed question. Path:'{}' Line: '{}'", path, line);
        return None;
    }
    let question: String = remove_exra_whitespaces(split_q[0]);
    let solutions: HashSet<String> = HashSet::from([remove_exra_whitespaces(split_q[1])]);
    Some(Question::new(path, question, solutions))
}

fn remove_exra_whitespaces(text: &str) -> String {
    MULTIPLE_WHITESPACES_REGEX.replace_all(text, " ").into_owned()
}

fn merge_answers(mut map: HashMap<String, Question>, new_q: Question) -> HashMap<String, Question> {
    // Merges answers for the same question.
    map.entry(new_q.question.clone())
        .and_modify(|old_q| {
            old_q.solutions = old_q.solutions.union(&new_q.solutions).cloned().collect()
        })  
        .or_insert(new_q);            
    map
} 


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn all_questions_are_parsed_from_nested_subdirectories() {
        // Given
        let path = PathBuf::from("tests/fixtures/nested");

        // When
        let question_map = read_all_questions_from(path);

        // Then
        assert_eq!(question_map.len(), 2);
    }
}
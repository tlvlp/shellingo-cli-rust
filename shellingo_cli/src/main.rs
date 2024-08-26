use std::{collections::HashSet, path::PathBuf};
use clap::{arg, command, value_parser};
use shellingo_core::question::Question;
use walkdir::{DirEntry, WalkDir};

const DEFAULT_PATH: &str = "questions";
const CLUE_PENALTY: i32 = 10;
const REVEAL_PENALTY:i32 = 5;

fn main() {
    let path = get_path_from_cli_args(); 
    
    println!("Loading questions from path: {}", path.display());
    let questions = read_all_questions_from(path);

}

fn get_path_from_cli_args() -> PathBuf {
    command!()
        .arg(arg!(<path> "Path to the questions. Either a file or a directory.")
           .required(false)
           .default_value(DEFAULT_PATH)
           .value_parser(value_parser!(PathBuf))
        )
        .get_matches()
        .get_one::<PathBuf>("path")
        .expect("Required field!")
        .to_owned()
}

fn read_all_questions_from(path: PathBuf) -> HashSet<Question> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .flat_map(read_lines_in_file)
            .map(parse_question_from_line)
            .collect()
}

fn read_lines_in_file(entry: DirEntry) -> HashSet<String> {
    todo!()
            // For example, you can read the file here
            // let content = fs::read_to_string(path).expect("Could not read file");
            // println!("File content: {}", content);
}

fn parse_question_from_line(line: String) -> Question {
    todo!()
}

use std::{collections::HashSet, path::PathBuf};
use clap::{arg, command, value_parser};
use shellingo_core::question::Question;

const DEFAULT_PATH: &str = "questions";
const CLUE_PENALTY: i32 = 10;
const REVEAL_PENALTY:i32 = 5;

fn main() {
    // Get the questions' path
    let path = command!()
        .arg(arg!(<path> "Path to the questions. Either a file or a directory.")
           .required(false)
           .default_value(DEFAULT_PATH)
           .value_parser(value_parser!(PathBuf))
        )
        .get_matches()
        .get_one::<PathBuf>("path")
        .expect("Required field!")
        .to_owned();
    let path_str = path.display();
    println!("Loading questions from path: {}", path_str);
    
    let questions = get_all_questions_at_path(path);

}

fn get_all_questions_at_path(path: PathBuf) -> HashSet<Question> {
    todo!()
}

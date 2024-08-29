mod question_parser;
use std::path::PathBuf;

use clap::{arg, command, value_parser};
use question_parser::read_all_questions_from;

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
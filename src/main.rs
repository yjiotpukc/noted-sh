use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead, BufReader, Result, Write};

const FIRST_COLUMN_COLOR: &str = "\x1B[38;5;163m";
const RESET_COLOR: &str = "\x1B[39;00m";

fn main() -> Result<()> {
    process_input()
}

fn process_input() -> Result<()> {
    let stdin = io::stdin();
    let input = BufReader::new(stdin);

    let mut stdout = io::stdout();

    let mut processed_line;
    for line in input.lines() {
        processed_line = process_line(line.expect("Error reading line")) + "\n";
        stdout
            .write_all(processed_line.as_bytes())
            .expect("Error writing line");
    }

    Ok(())
}

fn process_line(line: String) -> String {
    process_table(line)
}

fn process_table(line: String) -> String {
    lazy_static! {
        static ref MATCH_RE: Regex =
            Regex::new(r"^\s*.*?\s*(\|\s*.*?\s*)+$").expect("Regular expression is invalid");
        static ref REPLACE_RE: Regex =
            Regex::new(r"^(?P<fcpre>\s*)(?P<fc>.*?)(?P<fcpost>\s*)\|(?P<rem>.*?)$").unwrap();
    }

    if !MATCH_RE.is_match(&line) {
        return line;
    }

    let replace_string = format!("$fcpre{}$fc{}$fcpost│$rem", FIRST_COLUMN_COLOR, RESET_COLOR);
    REPLACE_RE.replace_all(&line, replace_string).to_string()
}

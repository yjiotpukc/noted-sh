use std::io::{self, BufRead, BufReader, Result, Write};

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
    line
}

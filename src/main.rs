use clap::{Command, Arg};
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
struct Record {
    line: usize,
    text: String,
}

fn process_file <P: AsRef<Path>>(p: P, re: Regex) -> Result<Vec<Record>, String> {

    let mut matches = Vec::new();
    let file_data = std::fs::read(p).map_err(|_| "Could not read string".to_string())?;

    if let Ok(file_text) = String::from_utf8(file_data) {
        for (line, text) in file_text.lines().enumerate() {

           if re.is_match(text) {
                matches.push(Record {
                    line,
                    text: text.to_string()
                })
            }
        }
    }

    Ok(matches)
}

fn main() -> Result<(), String> {

    let command = Command::new("pgrep")
        .about("A text filter given a regex")
        .author("Santiago Sanchez Taborda")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("File to be proccesed")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("Pattern to use to filter document")
                .takes_value(true)
                .required(true)
        ).get_matches();

    let pattern = Regex::new(command.value_of("pattern").unwrap()).map_err(|_| "Bad regex".to_string())?;
    
    let file_matches = process_file(command.value_of("file").ok_or("No file chosen")?, pattern);

    println!("{:?}", file_matches);

    Ok(())
}

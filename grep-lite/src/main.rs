use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

fn search(search_string: &str, file: File) -> Vec<String>{
  let re = Regex::new(search_string).unwrap();
  let reader = BufReader::new(file);

  let mut result: Vec<String> = Vec::new();
  let mut line_num: u32 = 0;
  for line_ in reader.lines() {
    let line = line_.unwrap();
    line_num += 1;
    let contains_substring = re.find(&line);
    match contains_substring {
      Some(_) => result.push(format!("{}: {}", line_num, line.trim())),
      None => (),
    }
  }
  result
}

fn main() {
  let args = App::new("GREP-Lite")
    .version("1.0")
    .author("Mark")
    .about("Searches for patterns in files")
    .arg(Arg::with_name("pattern")
      .help("The pattern to search for")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("input")
      .help("File to search")
      .takes_value(true)
      .required(true))
    .get_matches();

  let pattern = args.value_of("pattern").unwrap();
  let input = args.value_of("input").unwrap();
  let file = File::open(input).unwrap();

  let result = search(pattern, file);
  for line in result {
    println!("{}", line);
  }
}
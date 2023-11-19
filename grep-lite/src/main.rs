use regex::Regex;
use clap::{App,Arg};

fn search(search_string: &str, quote: &str) -> Vec<String>{
  let re = Regex::new(search_string).unwrap();
  let mut result: Vec<String> = Vec::new();
  let mut line_num: u32 = 0;
  for line in quote.lines() {
    line_num += 1;
    let contains_substring = re.find(line);
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
    .get_matches();

  let quote = "Every face, every shop, bedroom window, public-house, and
  dark square is a picture feverishly turned--in search of what?
  It is the same with books. What do we seek through millions of pages?";

  let pattern = args.value_of("pattern").unwrap();

  let result = search(pattern, quote);
  for line in result {
    println!("{}", line);
  }
}
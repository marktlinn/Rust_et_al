use regex::Regex;

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
  let quote = "Every face, every shop, bedroom window, public-house, and
  dark square is a picture feverishly turned--in search of what?
  It is the same with books. What do we seek through millions of pages?";

  let search_string = "of";

  let result = search(search_string, quote);
  for line in result {
    println!("{}", line);
  }
}
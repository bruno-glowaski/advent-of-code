// cargo-deps: regex
extern crate regex;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"\d").unwrap();

    let total = std::io::stdin()
        .lines()
        .filter_map(|i| parse_line(&i.ok()?, &regex))
        .reduce(|acc, i| acc + i)
        .unwrap();
    println!("{}", total);
}

fn parse_line(line: &str, regex: &Regex) -> Option<usize> {
    let mut iter = regex.captures_iter(&line).collect::<Vec<_>>();
    let digit0 = iter.first()?.get(0).unwrap().as_str();
    let digit1 = iter.last().unwrap().get(0).unwrap().as_str();
    Some(format!("{}{}", digit0, digit1).parse().unwrap())
}

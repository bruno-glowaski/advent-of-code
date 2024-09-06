// cargo-deps: regex
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut digit_map = HashMap::new();
    digit_map.insert("one", 1);
    digit_map.insert("two", 2);
    digit_map.insert("three", 3);
    digit_map.insert("four", 4);
    digit_map.insert("five", 5);
    digit_map.insert("six", 6);
    digit_map.insert("seven", 7);
    digit_map.insert("eight", 8);
    digit_map.insert("nine", 9);

    let spelled_digits_regex = digit_map.keys().copied().collect::<Vec<_>>().join("|");
    let reversed_spelled_digits_regex = reverse_str(&spelled_digits_regex);

    let regex = Regex::new(&(spelled_digits_regex + r"|\d")).unwrap();
    let reversed_regex = Regex::new(&(reversed_spelled_digits_regex + r"|\d")).unwrap();

    let total = std::io::stdin()
        .lines()
        .filter_map(|i| parse_line(&i.ok()?, &regex, &reversed_regex, &digit_map))
        .reduce(|acc, i| acc + i)
        .unwrap();
    println!("{}", total);
}

fn parse_line(
    line: &str,
    regex: &Regex,
    reversed_regex: &Regex,
    digit_map: &HashMap<&'static str, usize>,
) -> Option<usize> {
    let reversed_line = line.chars().rev().collect::<String>();
    let digit0 = parse_digit(regex.find(&line)?.as_str(), digit_map);
    let digit1 = parse_digit(
        &reverse_str(reversed_regex.find(&reversed_line).unwrap().as_str()),
        &digit_map,
    );
    Some(digit0 * 10 + digit1)
}

fn parse_digit(raw: &str, digit_map: &HashMap<&'static str, usize>) -> usize {
    digit_map
        .get(&raw)
        .copied()
        .or_else(|| raw.parse().ok())
        .unwrap()
        .clone()
}

fn reverse_str(src: &str) -> String {
    src.chars().rev().collect()
}

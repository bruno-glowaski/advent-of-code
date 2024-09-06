// cargo-deps: regex
extern crate regex;

use regex::Regex;

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

#[derive(Default)]
struct GameSet(usize, usize, usize);

fn main() {
    let parser = GameParser::default();

    let sum: usize = std::io::stdin()
        .lines()
        .filter_map(|l| parser.parse(&l.ok()?))
        .filter(|g| {
            g.sets
                .iter()
                .all(|set| set.0 <= 12 && set.1 <= 13 && set.2 <= 14)
        })
        .map(|g| g.id)
        .sum();
    println!("{}", sum);
}

struct GameParser {
    preffix_regex: Regex,
    set_separator: &'static str,
    cube_regex: Regex,
}

impl GameParser {
    pub fn default() -> Self {
        Self {
            preffix_regex: Regex::new(r"Game (\d+): ").unwrap(),
            set_separator: ";",
            cube_regex: Regex::new(r"(\d+) (blue|red|green)").unwrap(),
        }
    }

    pub fn parse(&self, src: &str) -> Option<Game> {
        let preffix_capture = self.preffix_regex.captures(&src)?;
        let id: usize = preffix_capture.get(1).unwrap().as_str().parse().unwrap();
        let preffix_len = preffix_capture.get(0).unwrap().len();
        let remainder = &src[preffix_len..];

        let sets = remainder
            .split(self.set_separator)
            .map(|s| self.parse_set(s))
            .collect();

        Some(Game { id, sets })
    }

    fn parse_set(&self, src: &str) -> GameSet {
        let mut set = GameSet::default();
        for captures in self.cube_regex.captures_iter(&src) {
            let count = captures.get(1).unwrap().as_str().parse().unwrap();
            match captures.get(2).unwrap().as_str() {
                "red" => set.0 = count,
                "green" => set.1 = count,
                "blue" => set.2 = count,
                _ => unreachable!(),
            };
        }
        set
    }
}

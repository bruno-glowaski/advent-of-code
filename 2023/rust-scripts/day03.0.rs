#!/usr/bin/env run-cargo-script

#[derive(Debug, Default)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    span: Span,
}

#[derive(Debug)]
struct Symbol {
    kind: char,
    position: Position,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Span {
    start: Position,
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

const EMPTY_SPACE: char = '.';

fn main() {
    let schematic = parse_schematic(std::io::stdin().lines().filter_map(|line| line.ok()));
    let sum: usize = schematic.part_numbers().map(|number| number.value).sum();
    println!("{}", sum);
}

fn parse_schematic(iter: impl Iterator<Item = String>) -> Schematic {
    let mut schematic = Schematic::default();
    for (row, line) in iter.enumerate() {
        let mut line_iter = line.char_indices().peekable();
        while let Some((col, c)) = line_iter.peek() {
            if *c == EMPTY_SPACE {
                let _ = line_iter.next();
                continue;
            }
            let position = Position { row, col: *col };
            if c.is_digit(10) {
                schematic
                    .numbers
                    .push(parse_number(&mut line_iter, position));
            } else {
                schematic
                    .symbols
                    .push(parse_symbol(&mut line_iter, position));
            }
        }
    }
    schematic
}

fn parse_number(
    iter: &mut (impl Iterator<Item = (usize, char)> + Clone),
    start: Position,
) -> Number {
    let value_str = iter
        .clone()
        .map(|(_, c)| c)
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    let _ = iter.skip(value_str.len() - 1).next();
    Number {
        value: value_str.parse().unwrap(),
        span: Span {
            start,
            len: value_str.len(),
        },
    }
}

fn parse_symbol(iter: &mut impl Iterator<Item = (usize, char)>, position: Position) -> Symbol {
    Symbol {
        kind: iter.next().unwrap().1,
        position,
    }
}

impl Schematic {
    pub fn part_numbers(&self) -> impl Iterator<Item = &Number> {
        self.symbols
            .iter()
            .flat_map(|s| {
                self.numbers
                    .iter()
                    .filter(move |n| n.span.is_adjacent_to_position(&s.position))
            })
            .collect::<std::collections::HashSet<&Number>>()
            .into_iter()
    }
}

impl Span {
    pub fn is_adjacent_to_position(&self, position: &Position) -> bool {
        let row_start = self.start.row.checked_sub(1).unwrap_or(0);
        let row_end = self.start.row + 1;
        let col_start = self.start.col.checked_sub(1).unwrap_or(0);
        let col_end = self.start.col + self.len;
        (row_start..=row_end).contains(&position.row)
            && (col_start..=col_end).contains(&position.col)
    }
}

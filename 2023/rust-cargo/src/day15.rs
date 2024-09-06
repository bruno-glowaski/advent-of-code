use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let commands = input.split(',').map(|src| {
        let mut iter = src.split(&['=', '-']);
        let label = iter.next().unwrap();
        let label_len = label.len();
        match &src[label_len..label_len + 1] {
            "=" => Command::Upsert {
                lens: (label, iter.next().unwrap().parse().unwrap()),
            },
            "-" => Command::Remove { label },
            _ => unreachable!(),
        }
    });
    let mut hashmap = vec![Vec::new(); 256];
    for command in commands {
        match command {
            Command::Upsert { lens } => {
                let bucket = &mut hashmap[hash(lens.0)];
                if let Some(position) = bucket.iter().position(|&(l, _)| l == lens.0) {
                    bucket[position].1 = lens.1;
                } else {
                    bucket.push(lens);
                }
            }
            Command::Remove { label } => hashmap[hash(label)].retain(|&(l, _)| l != label),
        }
    }
    hashmap
        .into_iter()
        .enumerate()
        .flat_map(|(b, bucket)| {
            bucket
                .into_iter()
                .enumerate()
                .map(move |(s, (_, f))| (b + 1) * (s + 1) * f)
        })
        .sum()
}

enum Command<'a> {
    Remove { label: &'a str },
    Upsert { lens: (&'a str, usize) },
}

fn hash(src: &str) -> usize {
    let mut current_value = 0;
    for c in src.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn hash_sample() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    pub fn part1_sample() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    pub fn part2_sample() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }
}

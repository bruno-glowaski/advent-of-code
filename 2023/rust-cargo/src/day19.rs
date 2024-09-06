use std::{cmp::Ordering, collections::HashMap, ops::RangeInclusive};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    X,
    M,
    A,
    S,
}

impl From<&str> for Attribute {
    fn from(s: &str) -> Self {
        use Attribute::*;
        match s {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Action<'s> {
    Workflow(&'s str),
    Accept,
    Reject,
}

impl<'s> From<&'s str> for Action<'s> {
    fn from(value: &'s str) -> Self {
        match value {
            "A" => Action::Accept,
            "R" => Action::Reject,
            s => Action::Workflow(s),
        }
    }
}

#[derive(Debug)]
pub struct Rule<'s> {
    attr: Attribute,
    condition: Ordering,
    value: usize,
    next: Action<'s>,
}

impl<'s> From<&'s str> for Rule<'s> {
    fn from(value: &'s str) -> Self {
        let mut iter = value.split(&['<', '>', ':']);
        let attr = iter.next().unwrap();
        let condition = match &value[attr.len()..attr.len() + 1] {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => panic!(),
        };
        let attr = attr.into();
        let value = iter.next().unwrap().parse().unwrap();
        let next = iter.next().unwrap().into();
        Self {
            attr,
            condition,
            value,
            next,
        }
    }
}

impl Rule<'_> {
    pub fn try_apply(&self, part: &Part<usize>) -> Option<&Action<'_>> {
        (part.attr(self.attr).cmp(&self.value) == self.condition).then_some(&self.next)
    }

    fn apply_over(
        &self,
        part_range: PartRange,
    ) -> (Option<(PartRange, &Action<'_>)>, Option<PartRange>) {
        let part_attr = part_range.attr(self.attr);
        if part_attr.contains(&self.value) {
            let (applicable, remainder) = match self.condition {
                Ordering::Less => (
                    *part_attr.start()..=self.value - 1,
                    self.value..=*part_attr.end(),
                ),
                Ordering::Greater => (
                    self.value + 1..=*part_attr.end(),
                    *part_attr.start()..=self.value,
                ),
                _ => panic!(),
            };
            (
                Some((
                    part_range.clone().with_attr(self.attr, applicable),
                    &self.next,
                )),
                Some(part_range.with_attr(self.attr, remainder)),
            )
        } else {
            if self.condition == Ordering::Less && *part_attr.end() < self.value {
                (Some((part_range, &self.next)), None)
            } else if self.condition == Ordering::Greater && *part_attr.start() > self.value {
                (Some((part_range, &self.next)), None)
            } else {
                (None, Some(part_range))
            }
        }
    }
}

#[derive(Debug)]
pub struct Workflow<'s> {
    rules: Vec<Rule<'s>>,
    default: Action<'s>,
}

impl<'s> From<&'s str> for Workflow<'s> {
    fn from(value: &'s str) -> Self {
        let mut iter = value.split(',');
        let default = iter.next_back().unwrap().into();
        let rules = iter.map(Into::into).collect();
        Self { default, rules }
    }
}

impl Workflow<'_> {
    pub fn apply(&self, part: &Part<usize>) -> &Action<'_> {
        self.rules
            .iter()
            .find_map(|rule| rule.try_apply(part))
            .unwrap_or(&self.default)
    }

    fn apply_over<'a>(
        &'a self,
        mut part_range: Part<RangeInclusive<usize>>,
        accepted_ranges: &mut Vec<PartRange>,
        remaining_ranges: &mut Vec<(&'a str, PartRange)>,
    ) {
        for rule in &self.rules {
            let (applicable, remainder) = rule.apply_over(part_range);
            match applicable {
                Some((part_range, Action::Accept)) => accepted_ranges.push(part_range),
                Some((part_range, Action::Workflow(workflow))) => {
                    remaining_ranges.push((workflow, part_range))
                }
                _ => {}
            }
            match remainder {
                Some(remainder) => part_range = remainder,
                None => return,
            }
        }
        match self.default {
            Action::Workflow(workflow) => remaining_ranges.push((workflow, part_range)),
            Action::Accept => accepted_ranges.push(part_range),
            Action::Reject => {}
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Part<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

type PartRange = Part<RangeInclusive<usize>>;

impl From<&str> for Part<usize> {
    fn from(value: &str) -> Self {
        let mut values = value.split(',').map(|attr| attr[2..].parse().unwrap());
        Self {
            x: values.next().unwrap(),
            m: values.next().unwrap(),
            a: values.next().unwrap(),
            s: values.next().unwrap(),
        }
    }
}

impl<T> Part<T> {
    pub fn attr(&self, attr: Attribute) -> &T {
        use Attribute::*;
        match attr {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }

    pub fn attr_mut(&mut self, attr: Attribute) -> &mut T {
        use Attribute::*;
        match attr {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }

    pub fn with_attr(mut self, attr: Attribute, value: T) -> Self {
        *self.attr_mut(attr) = value;
        self
    }
}

impl Part<usize> {
    pub fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl PartRange {
    pub fn total_accepted(&self) -> usize {
        [&self.x, &self.a, &self.m, &self.s]
            .into_iter()
            .cloned()
            .map(Iterator::count)
            .product()
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let workflows = lines
        .by_ref()
        .take_while(|l| l.len() > 0)
        .map(|l| {
            let mut iter = l.split(&['{', '}']);
            let name = iter.next().unwrap();
            let workflow = Workflow::from(iter.next().unwrap());
            (name, workflow)
        })
        .collect::<HashMap<_, _>>();
    let parts = lines.map(|l| Part::from(&l[1..l.len() - 1]));

    parts
        .filter(|part| {
            let mut current_workflow = &workflows["in"];
            loop {
                match current_workflow.apply(part) {
                    Action::Accept => break true,
                    Action::Reject => break false,
                    Action::Workflow(new_workflow) => current_workflow = &workflows[new_workflow],
                }
            }
        })
        .map(|p| p.total_rating())
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    const INITIAL_RANGE: RangeInclusive<usize> = 1..=4000;

    let mut lines = input.lines();
    let workflows = lines
        .by_ref()
        .take_while(|l| l.len() > 0)
        .map(|l| {
            let mut iter = l.split(&['{', '}']);
            let name = iter.next().unwrap();
            let workflow = Workflow::from(iter.next().unwrap());
            (name, workflow)
        })
        .collect::<HashMap<_, _>>();

    let mut part_ranges = vec![(
        "in",
        Part {
            x: INITIAL_RANGE,
            a: INITIAL_RANGE,
            m: INITIAL_RANGE,
            s: INITIAL_RANGE,
        },
    )];
    let mut accepted_ranges = Vec::new();
    while let Some((workflow, part_range)) = part_ranges.pop() {
        workflows[&workflow].apply_over(part_range, &mut accepted_ranges, &mut part_ranges);
    }
    accepted_ranges
        .into_iter()
        .map(|p| p.total_accepted())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1(SAMPLE), 19114);
    }

    #[test]
    pub fn part2_sample() {
        assert_eq!(part2(SAMPLE), 167409079868000);
    }
}

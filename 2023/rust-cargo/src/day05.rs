use std::{cmp::Ordering, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct AlmanacPart1 {
    seeds: Vec<usize>,
    mappers: AlmanacMappers,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlmanacPart2 {
    seeds: Vec<Range<usize>>,
    mappers: AlmanacMappers,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlmanacMappers {
    seed_to_soil: RangeMaps,
    soil_to_fertilizer: RangeMaps,
    fertilizer_to_water: RangeMaps,
    water_to_light: RangeMaps,
    light_to_temperature: RangeMaps,
    temperature_to_humidity: RangeMaps,
    humidity_to_location: RangeMaps,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RangeMaps(Vec<RangeMap>);

#[derive(Debug, PartialEq, Eq)]
pub struct RangeMap {
    src_start: usize,
    dst_start: usize,
    len: usize,
}

#[aoc_generator(day5, part1)]
pub fn input_generator_part1(src: &str) -> AlmanacPart1 {
    let mut iter = src.lines();

    let seeds = parse_seeds(&mut iter);

    iter.next();
    iter.next();

    let mappers = parse_mappers(&mut iter);

    AlmanacPart1 { seeds, mappers }
}

#[aoc_generator(day5, part2)]
pub fn input_generator_part2(src: &str) -> AlmanacPart2 {
    let mut iter = src.lines();

    let seeds = parse_seed_ranges(&mut iter);

    iter.next();
    iter.next();

    let mappers = parse_mappers(&mut iter);

    AlmanacPart2 { seeds, mappers }
}

fn parse_seeds<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
    iter.next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_seed_ranges<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<Range<usize>> {
    let mut iter = iter
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap);
    let mut result = Vec::with_capacity(iter.clone().count() / 2);
    while let Some(start) = iter.next() {
        let len = iter.next().unwrap();
        result.push(start..start + len);
    }
    result
}

fn parse_mappers<'a>(iter: &mut impl Iterator<Item = &'a str>) -> AlmanacMappers {
    let seed_to_soil = parse_map_ranges(iter);

    iter.next();

    let soil_to_fertilizer = parse_map_ranges(iter);

    iter.next();

    let fertilizer_to_water = parse_map_ranges(iter);

    iter.next();

    let water_to_light = parse_map_ranges(iter);

    iter.next();

    let light_to_temperature = parse_map_ranges(iter);

    iter.next();

    let temperature_to_humidity = parse_map_ranges(iter);

    iter.next();

    let humidity_to_location = parse_map_ranges(iter);

    AlmanacMappers {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn parse_map_ranges<'a>(iter: &mut impl Iterator<Item = &'a str>) -> RangeMaps {
    iter.take_while(|line| !line.is_empty())
        .map(parse_map_range)
        .collect()
}

fn parse_map_range(line: &str) -> RangeMap {
    let mut iter = line.split_whitespace().map(str::parse).map(Result::unwrap);
    let dst_start = iter.next().unwrap();
    let src_start = iter.next().unwrap();
    let len = iter.next().unwrap();
    RangeMap {
        dst_start,
        src_start,
        len,
    }
}

impl FromIterator<RangeMap> for RangeMaps {
    fn from_iter<T: IntoIterator<Item = RangeMap>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

#[aoc(day5, part1)]
pub fn part1(almanac: &AlmanacPart1) -> usize {
    almanac
        .seeds
        .iter()
        .copied()
        .map(|s| almanac.mappers.map_seed_to_location(s))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(almanac: &AlmanacPart2) -> usize {
    almanac
        .mappers
        .map_seed_ranges_to_location_ranges(almanac.seeds.iter().cloned())
        .map(|r| r.start)
        .min()
        .unwrap()
}

impl AlmanacMappers {
    #[inline]
    pub fn map_seed_ranges_to_location_ranges(
        &self,
        seed_ranges: impl Iterator<Item = Range<usize>>,
    ) -> impl Iterator<Item = Range<usize>> {
        let mut buffer_0 = Vec::from_iter(seed_ranges);
        let mut buffer_1 = Vec::with_capacity(buffer_0.capacity());
        for mapper in self.seed_to_location_mapping_sequence() {
            while let Some(input) = buffer_0.pop() {
                for output in mapper.map_range_or_keep(input) {
                    buffer_1.push(output);
                }
            }
            std::mem::swap(&mut buffer_0, &mut buffer_1);
        }
        buffer_0.into_iter()
    }

    #[inline]
    pub fn map_seed_to_location(&self, seed: usize) -> usize {
        self.seed_to_location_mapping_sequence()
            .fold(seed, |seed, mapper| mapper.map_or_keep(seed))
    }

    #[inline]
    fn seed_to_location_mapping_sequence(&self) -> impl Iterator<Item = &RangeMaps> {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .into_iter()
    }
}

impl RangeMaps {
    pub fn map_or_keep(&self, src: usize) -> usize {
        self.0
            .binary_search_by(|r| r.cmp(src))
            .map(|i| self.0[i].map(src))
            .unwrap_or(src)
    }

    pub fn map_range_or_keep(&self, src: Range<usize>) -> impl Iterator<Item = Range<usize>> {
        let mappers_start = self
            .0
            .binary_search_by(|r| r.cmp(src.start))
            .unwrap_or_else(|e| e);
        let mappers_end = self
            .0
            .binary_search_by(|r| r.cmp(src.end))
            .unwrap_or_else(|e| e - 1)
            + 1;
        let mappers_range = mappers_start..mappers_end;
        let mut ranges = Vec::with_capacity(1 + mappers_range.len() * 2);
        let mut remainder = Some(src);
        for mapper in &self.0[mappers_start..mappers_end] {
            let (before, inside, after) = mapper.map_overlap(remainder.unwrap()).unwrap();
            if let Some(before) = before {
                ranges.push(before);
            }
            ranges.push(inside);
            remainder = after;
        }
        if let Some(remainder) = remainder {
            ranges.push(remainder);
        }
        ranges.into_iter()
    }
}

impl RangeMap {
    pub fn src_end(&self) -> usize {
        self.src_start + self.len
    }

    pub fn src_range(&self) -> Range<usize> {
        self.src_start..self.src_end()
    }

    pub fn dst_end(&self) -> usize {
        self.dst_start + self.len
    }

    pub fn dst_range(&self) -> Range<usize> {
        self.dst_start..self.dst_end()
    }

    pub fn map(&self, src: usize) -> usize {
        self.dst_start + (src - self.src_start)
    }

    pub fn map_overlap(&self, src: Range<usize>) -> RangeOverlap {
        match self.overlap_with_src(src) {
            None => None,
            Some((before, inside, after)) => Some((before, self.map_inside(inside), after)),
        }
    }

    pub fn overlap_with_src(&self, src: Range<usize>) -> RangeOverlap {
        if !self.intersects_with_range(&src) {
            return None;
        }
        let Range {
            start: a_min,
            end: a_max,
        } = src;
        let Range {
            start: b_min,
            end: b_max,
        } = self.src_range();
        let inside = a_min.max(b_min)..a_max.min(b_max);
        let before = (a_min < b_min).then(|| a_min..b_min);
        let after = (a_max > b_max).then(|| b_max..a_max);
        Some((before, inside, after))
    }

    pub fn cmp(&self, rhs: usize) -> Ordering {
        if self.src_range().contains(&rhs) {
            Ordering::Equal
        } else {
            if rhs < self.src_start {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }

    pub fn intersects_with_range(&self, target: &Range<usize>) -> bool {
        target.start < self.src_end() && target.end > self.src_start
    }

    pub fn map_inside(&self, src: Range<usize>) -> Range<usize> {
        self.overflowing_map(src.start)..self.overflowing_map(src.end)
    }

    pub fn overflowing_map(&self, src: usize) -> usize {
        self.dst_start
            .wrapping_add(src.wrapping_sub(self.src_start))
    }
}

type RangeOverlap = Option<(Option<Range<usize>>, Range<usize>, Option<Range<usize>>)>;

impl From<(usize, usize, usize)> for RangeMap {
    fn from((dst_start, src_start, len): (usize, usize, usize)) -> Self {
        RangeMap {
            dst_start,
            src_start,
            len,
        }
    }
}

impl From<Vec<RangeMap>> for RangeMaps {
    fn from(mut value: Vec<RangeMap>) -> Self {
        value.sort_by_key(|r| r.src_start);
        RangeMaps(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    fn parsed_sample_part1() -> AlmanacPart1 {
        super::input_generator_part1(SAMPLE_INPUT)
    }

    fn parsed_sample_part2() -> AlmanacPart2 {
        super::input_generator_part2(SAMPLE_INPUT)
    }

    #[test]
    fn input_generator_part1_sample() {
        assert_eq!(
            parsed_sample_part1(),
            AlmanacPart1 {
                seeds: vec![79, 14, 55, 13],
                mappers: AlmanacMappers {
                    seed_to_soil: vec![(50, 98, 2).into(), (52, 50, 48).into()].into(),
                    soil_to_fertilizer: vec![
                        (0, 15, 37).into(),
                        (37, 52, 2).into(),
                        (39, 0, 15).into()
                    ]
                    .into(),
                    fertilizer_to_water: vec![
                        (49, 53, 8).into(),
                        (0, 11, 42).into(),
                        (42, 0, 7).into(),
                        (57, 7, 4).into()
                    ]
                    .into(),
                    water_to_light: vec![(88, 18, 7).into(), (18, 25, 70).into(),].into(),
                    light_to_temperature: vec![
                        (45, 77, 23).into(),
                        (81, 45, 19).into(),
                        (68, 64, 13).into(),
                    ]
                    .into(),
                    temperature_to_humidity: vec![(0, 69, 1).into(), (1, 0, 69).into(),].into(),
                    humidity_to_location: vec![(60, 56, 37).into(), (56, 93, 4).into(),].into()
                }
            }
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&parsed_sample_part1()), 35);
    }

    #[test]
    fn input_generator_part2_sample() {
        assert_eq!(parsed_sample_part2().seeds, vec![79..93, 55..68],);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&parsed_sample_part2()), 46);
    }

    #[test]
    fn map_overlap() {
        let map_range = RangeMap::from((20, 10, 20));
        assert_eq!(map_range.map_overlap(10..30), Some((None, (20..40), None)));
        assert_eq!(map_range.map_overlap(15..25), Some((None, (25..35), None)));
        assert_eq!(
            map_range.map_overlap(0..20),
            Some((Some(0..10), (20..30), None))
        );
        assert_eq!(
            map_range.map_overlap(20..40),
            Some((None, 30..40, Some(30..40)))
        );
        assert_eq!(
            map_range.map_overlap(0..40),
            Some((Some(0..10), 20..40, Some(30..40)))
        );
        assert_eq!(map_range.map_overlap(0..5), None,);
        assert_eq!(map_range.map_overlap(30..40), None,);
    }

    #[test]
    fn map_range_or_keep() {
        let map_ranges =
            RangeMaps::from_iter([RangeMap::from((50, 98, 2)), RangeMap::from((52, 50, 48))]);
        assert_eq!(
            map_ranges.map_range_or_keep(98..104).collect::<Vec<_>>(),
            vec![50..52, 100..104]
        )
    }
}

use aoc_core::includes::Includes;
use aoc_core::set::range_set::RangeSet;
use aoc_core::set::{SetDifference, SetIntersection, SetUnion};
use std::ops::Range;

pub struct SeedAlmanac {
    seeds: Vec<u64>,
    maps: Vec<SeedMap>,
}

impl SeedAlmanac {
    pub fn to_location_individual(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|s| self.maps.iter().fold(*s, |acc, val| val.map(&acc)))
            .collect()
    }

    pub fn to_location_range_min(&self) -> u64 {
        let seed_ranges = self
            .seeds
            .chunks(2)
            .map(|v| {
                let start = v.get(0).unwrap();
                let size = v.get(1).unwrap();
                *start..start + size
            })
            .collect();
        let mut range_set = RangeSet::new(seed_ranges);
        for map in self.maps.iter() {
            let next_range_set = map.map_range_set(&range_set);
            range_set = next_range_set
        }
        range_set.iter().next().unwrap().start
    }
}

impl From<&str> for SeedAlmanac {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");
        let seeds = split
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|i| i.parse().unwrap())
            .collect();
        let maps = split.map(|i| i.into()).collect();

        SeedAlmanac { seeds, maps }
    }
}

#[derive(Clone)]
struct SeedMap {
    source_name: String,
    dest_name: String,
    mappings: Vec<SeedMapping>,
}

impl SeedMap {
    fn map(&self, input: &u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(v) = mapping.map(input) {
                return v;
            }
        }
        *input
    }

    fn to_range_set(&self) -> RangeSet<u64> {
        let source_ranges = self.mappings.iter().map(|x| x.source.clone()).collect();
        RangeSet::new(source_ranges)
    }

    fn map_range_set(&self, lhs: &RangeSet<u64>) -> RangeSet<u64> {
        let rhs = self.to_range_set();
        // the ranges that will have some transformation applied below
        let will_map = lhs.intersection(&rhs);
        // the ranges that will have no transformation applied
        let unmapped = lhs.difference(&will_map);
        // the ranges with transformations applied in this seed map's mappings
        let mut mapped = vec![];
        'outer: for r in will_map.iter() {
            for m in self.mappings.iter() {
                if let Some(u) = m.shift_range(r) {
                    mapped.push(u);
                    continue 'outer;
                }
            }
            panic!(
                "All ranges in will_map should have been mapped!\n{}-to-{}, mapping {}..{} in {}",
                self.source_name, self.dest_name, r.start, r.end, rhs
            );
        }
        unmapped.union(&RangeSet::new(mapped))
    }
}

impl From<&str> for SeedMap {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let mut description = lines.next().unwrap().split(' ').next().unwrap().split('-');
        let source_name = description.next().unwrap().to_owned();
        let dest_name = description.last().unwrap().to_owned();
        let mappings = lines.map(|i| i.into()).collect();

        SeedMap {
            source_name,
            dest_name,
            mappings,
        }
    }
}

#[derive(Clone)]
struct SeedMapping {
    source: Range<u64>,
    dest: Range<u64>,
}

impl SeedMapping {
    fn map(&self, input: &u64) -> Option<u64> {
        if self.source.contains(input) {
            Some(*input - self.source.start + self.dest.start)
        } else {
            None
        }
    }

    fn shift_range(&self, rhs: &Range<u64>) -> Option<Range<u64>> {
        if !self.source.includes(rhs) {
            None
        } else {
            let abs_diff = self.source.start.abs_diff(self.dest.start);
            match self.source.start <= self.dest.start {
                true => Some(rhs.start + abs_diff..rhs.end + abs_diff),
                false => Some(rhs.start - abs_diff..rhs.end - abs_diff),
            }
        }
    }
}

impl From<&str> for SeedMapping {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let dest: u64 = split.next().unwrap().parse().unwrap();
        let source: u64 = split.next().unwrap().parse().unwrap();
        let range: u64 = split.next().unwrap().parse().unwrap();

        SeedMapping {
            dest: dest..dest + range,
            source: source..source + range,
        }
    }
}

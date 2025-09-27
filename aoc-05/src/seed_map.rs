use aoc_core::overlaps::Overlaps;
use std::ops::Range;
use std::thread;
use std::thread::JoinHandle;

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
        let seed_ranges = self.seeds.chunks(2).map(|v| {
            let start = v.get(0).unwrap();
            let size = v.get(1).unwrap();
            *start..start + size
        });
        let handles: Vec<JoinHandle<u64>> = seed_ranges
            .map(|r| {
                let maps = self.maps.clone();
                thread::spawn(move || {
                    r.into_iter()
                        .map(|u| maps.iter().fold(u, |acc, val| val.map(&acc)))
                        .min()
                        .unwrap()
                })
            })
            .collect();
        let mut min: Option<u64> = None;
        for h in handles {
            let r = h.join().unwrap();
            match min {
                None => min = Some(r),
                Some(c) => min = Some(c.min(r)),
            }
        }
        min.unwrap()
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

    // fn map_ranges(&self, input: &Vec<Range<u64>>) -> Vec<Range<u64>> {
    //     let r: Vec<Range<u64>> = input
    //         .iter()
    //         .flat_map(|i| self.mappings.iter().filter_map(|m| m.map_range(i)))
    //         .collect();
    //     // any inputs that aren't accounted for by a mapping need to map to themselves
    //     input.
    // }
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

    fn map_range(&self, input: &Range<u64>) -> Option<Range<u64>> {
        if input.overlaps(&self.source) {
            let result_start =
                self.source.start.max(input.start) - self.source.start + self.dest.start;
            let result_end = self.source.end.min(input.end) - self.source.start + self.dest.start;
            Some(result_start..result_end)
        } else {
            None
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

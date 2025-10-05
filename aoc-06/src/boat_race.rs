use std::str::Lines;

pub struct BoatRace {
    time: u64,
    distance: u64,
}

impl BoatRace {
    pub fn count_winning_holds(&self) -> u64 {
        (1..self.time)
            .into_iter()
            .map(|x| self.get_distance_for_hold(&x))
            .filter(|x| x > &self.distance)
            .count() as u64
    }

    fn get_distance_for_hold(&self, hold_for: &u64) -> u64 {
        if hold_for >= &self.time {
            return 0;
        }
        let runtime = self.time - hold_for;
        runtime * hold_for
    }
}

impl From<Lines<'_>> for BoatRace {
    fn from(value: Lines<'_>) -> Self {
        let mut iter = value.into_iter();
        let mapper = |item: &str| {
            item.split(':')
                .skip(1)
                .next()
                .unwrap()
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        };
        let time = mapper(iter.next().unwrap());
        let distance = mapper(iter.next().unwrap());
        BoatRace { time, distance }
    }
}

pub struct BoatRaceSeries {
    races: Vec<BoatRace>,
}

impl BoatRaceSeries {
    pub fn get_winning_race_product(&self) -> u64 {
        self.races
            .iter()
            .map(BoatRace::count_winning_holds)
            .fold(1, |acc, val| acc * val)
    }
}

impl From<Lines<'_>> for BoatRaceSeries {
    fn from(value: Lines<'_>) -> Self {
        let mut iter = value.into_iter();
        let mapper = |item: &str| {
            item.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>()
        };
        let times = mapper(iter.next().unwrap());
        let distances = mapper(iter.next().unwrap());
        let mut races = vec![];
        for (i, time) in times.iter().enumerate() {
            let distance = distances.get(i).unwrap();
            races.push(BoatRace {
                time: *time,
                distance: *distance,
            })
        }

        BoatRaceSeries { races }
    }
}

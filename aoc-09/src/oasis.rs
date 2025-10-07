use std::str::Lines;

struct OasisItem {
    history: Vec<i32>,
}

impl OasisItem {
    fn extrapolate_next(&self) -> i32 {
        let differences = self.compute_difference_list();
        let mut value = 0i32;
        for diff in differences.iter().rev() {
            value = diff.last().unwrap() + value;
        }
        value
    }

    fn extrapolate_previous(&self) -> i32 {
        let differences = self.compute_difference_list();
        let mut value = 0i32;
        for diff in differences.iter().rev() {
            value = diff.first().unwrap() - value;
        }
        value
    }

    fn compute_difference_list(&self) -> Vec<Vec<i32>> {
        let mut result = vec![self.history.clone()];
        while result.last().unwrap().iter().any(|x| *x != 0) {
            let next = OasisItem::compute_differences(result.last().unwrap());
            result.push(next);
        }

        result
    }

    fn compute_differences(source: &Vec<i32>) -> Vec<i32> {
        source
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }
}

impl From<&str> for OasisItem {
    fn from(value: &str) -> Self {
        let history = value.split(' ').map(|x| x.parse().unwrap()).collect();

        OasisItem { history }
    }
}

pub struct OasisReport {
    items: Vec<OasisItem>,
}

impl OasisReport {
    pub fn sum_next_values(&self) -> i32 {
        self.items.iter().map(|x| x.extrapolate_next()).sum()
    }

    pub fn sum_previous_values(&self) -> i32 {
        self.items.iter().map(|x| x.extrapolate_previous()).sum()
    }
}

impl From<Lines<'_>> for OasisReport {
    fn from(value: Lines<'_>) -> Self {
        let items = value.into_iter().map(|x| x.into()).collect();

        OasisReport { items }
    }
}

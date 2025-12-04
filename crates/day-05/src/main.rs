mod gimme_input;

fn main() {
    let fresh_ids = gimme_input::INPUT_FINAL_FRESH_IDS;
    let food_ids = gimme_input::INPUT_FINAL_FOOD_IDS;

    let (fresh_ids, food_ids) = parse(fresh_ids, food_ids);

    let fresh_count = part_2::solve(&fresh_ids);
    println!("{fresh_count}");
}

fn parse(fresh_ids: &str, food_ids: &str) -> (Vec<Range>, Vec<u64>) {
    let food_ids_vec = food_ids
        .lines()
        .map(|id| id.parse::<u64>().expect("Wack food id"))
        .collect();

    let fresh_ids_ranges = fresh_ids
        .lines()
        .map(|range| range.split("-").collect::<Vec<_>>())
        .map(|splits| {
            Range(
                splits[0].parse::<u64>().expect("Wack left range"),
                splits[1].parse::<u64>().expect("Wack right range"),
            )
        })
        .collect();

    (fresh_ids_ranges, food_ids_vec)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range(pub u64, pub u64);

impl Range {
    pub fn in_range(&self, n: u64) -> bool {
        self.0 <= n && n <= self.1
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

mod part_1 {
    use crate::Range;

    pub fn solve(fresh_ids: &[Range], food_ids: &[u64]) -> usize {
        food_ids
            .iter()
            .map(|food_id| fresh_ids.iter().any(|range| range.in_range(*food_id)))
            .filter(|&fresh| fresh)
            .count()
    }
}

mod part_2 {
    use std::collections::HashSet;

    use crate::Range;

    pub fn solve_naive(fresh_ids: &[Range]) -> usize {
        fresh_ids
            .iter()
            .flat_map(|range| range.0..=range.1)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn solve(fresh_ids: &[Range]) -> u64 {
        if fresh_ids.len() == 0 {
            return 0;
        }

        // ugh impossible to sort without mut since all Rust sorts require a mutable vector
        let sorted = {
            let mut v = fresh_ids.to_vec();
            // Order doesn't matter here anyway
            v.sort_unstable();
            v
        };

        // THE RECIPE
        // 1. The ranges are sorted so we can inspect on the very last element in our vector
        // 2. If the last range overlaps with our current range, we pop the current range and push a new extended range
        // 3. If no overlap, just add it since it's new and cool
        // 4. Then just take advantage of the end and start values to get the amount of numbers in that range. Possible to do
        // since we sorted our ranges and each all ranges are unique wrt their boundaries
        sorted
            .into_iter()
            .fold(vec![], |mut merged: Vec<Range>, Range(start, end)| {
                match merged.last() {
                    Some(&Range(last_start, last_end)) if start <= last_end + 1 => {
                        merged.pop();
                        merged.push(Range(last_start, last_end.max(end)));
                    }
                    _ => merged.push(Range(start, end)),
                }
                merged
            })
            .iter()
            .map(|Range(start, end)| end - start + 1)
            .sum()
    }
}

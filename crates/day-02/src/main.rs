#![feature(explicit_tail_calls)]

pub mod gimme_input;
pub mod math;

fn main() {
    let input = gimme_input::INPUT;
    let id_ranges = parse_ids(input);
    dbg!("{id_ranges:?}");

    let invalid_id_ranges = part_2::solve(&id_ranges);
    println!("{invalid_id_ranges}");
}

fn parse_ids(input: &str) -> Vec<IdRange> {
    input
        .split(",")
        .map(|range| range.split("-").collect::<Vec<_>>())
        .map(|chunk| IdRange {
            left: chunk[0].parse::<u64>().unwrap(),
            right: chunk[1].parse::<u64>().unwrap(),
        })
        .collect()
}

#[derive(Debug)]
struct IdRange {
    left: u64,
    right: u64,
}

mod part_1 {
    use crate::math::{digit_length, left_right_of_num, round_to_power_of_10};

    use super::*;

    fn solve(ids: &[IdRange]) -> u64 {
        ids.iter()
            .flat_map(|range| get_invalid_ids(range, range.left, &[]))
            .sum()
    }

    #[inline]
    pub fn invalid_ids_in_range(start: u64, end: u64) -> Vec<u64> {
        (start..=end)
            .filter(|&n| {
                let (left, right) = left_right_of_num(n);
                left == right
            })
            .collect()
    }

    fn get_invalid_ids(range: &IdRange, step: u64, invalid_ids: &[u64]) -> Vec<u64> {
        match step {
            n if n > range.right => vec![],
            n if !digit_length(n).is_multiple_of(2) => {
                let step = round_to_power_of_10(n);
                become get_invalid_ids(range, step, invalid_ids)
            }
            n if digit_length(n).is_multiple_of(2) => {
                let range_end = {
                    let next_power = round_to_power_of_10(n) - 1;
                    next_power.min(range.right)
                };
                let invalid_ids_in_range = invalid_ids_in_range(n, range_end);

                let next_invalid_range =
                    get_invalid_ids(range, round_to_power_of_10(n), invalid_ids);
                invalid_ids_in_range
                    .into_iter()
                    .chain(next_invalid_range)
                    .collect()
            }
            _ => {
                unreachable!("WHAT")
            }
        }
    }
}

mod part_2 {
    use super::*;

    pub fn solve(ids: &[IdRange]) -> u64 {
        ids.iter().flat_map(get_invalid_ids_any_sequence).sum()
    }

    fn get_invalid_ids_any_sequence(range: &IdRange) -> Vec<u64> {
        (range.left..=range.right)
            .filter_map(|n| {
                let num_as_arr = n
                    .to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                contains_sequence(1, &num_as_arr, n)
            })
            .collect()
    }

    fn contains_sequence(chunk_size: u64, num_as_arr: &[u32], num: u64) -> Option<u64> {
        match chunk_size {
            _n if chunk_size > num_as_arr.len() as u64 / 2 => None,
            n => {
                let chunks = num_as_arr.chunks(n as usize);
                let pattern = chunks.clone().next()?;
                if chunks.skip(1).all(|chunk| chunk == pattern) {
                    Some(num)
                } else {
                    become contains_sequence(n + 1, num_as_arr, num)
                }
            }
        }
    }
}

#![feature(explicit_tail_calls)]
use std::{collections::HashMap, time::Instant};

pub mod gimme_input;

fn main() {
    let input = gimme_input::INPUT;

    let start = Instant::now();

    let banks = parse(input);
    let power = part_2::solve(&banks);

    let solve_time = start.elapsed();
    println!("{power}");
    println!("Solve time: {:?}", solve_time);
}

fn parse(input: &str) -> Vec<PowerBank> {
    input
        .lines()
        .map(|line| line.chars())
        .map(|chars| {
            chars
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u64)
                .collect()
        })
        .map(PowerBank::new)
        .collect()
}

#[derive(Debug)]
struct PowerBank {
    banks: Vec<u64>,
    // Value of bank points to vec of indices for that value
    bank_vec: Vec<Vec<u64>>,
}

impl PowerBank {
    fn new(banks: Vec<u64>) -> Self {
        let max_value = *banks.iter().max().unwrap() as usize;
        let bank_vec: Vec<Vec<u64>> = banks.iter().enumerate().fold(
            vec![Vec::new(); max_value + 1],
            |mut vec, (idx, value)| {
                vec[*value as usize].push(idx as u64);
                vec
            },
        );
        Self { banks, bank_vec }
    }
}

mod part_1 {

    use super::*;

    pub fn solve(banks: &[PowerBank]) -> u64 {
        banks
            .iter()
            .map(|bank| max_in_powerbank(bank))
            //       .inspect(|max| println!("{max}"))
            .sum()
    }

    // it's linear search. Too much time spent trying to be smort
    fn max_in_powerbank(power_bank: &PowerBank) -> u64 {
        power_bank
            .bank_vec
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(index, v)| {
                if !v.is_empty() {
                    max_in_range(v[0], &power_bank.banks)
                        .and_then(|m| format!("{index}{m}").parse::<u64>().ok())
                } else {
                    None
                }
            })
            .max()
            .expect("WTF")
    }

    fn max_in_range(start: u64, vec: &[u64]) -> Option<u64> {
        let start_range = (start + 1) as usize;
        if start_range < vec.len() {
            return vec[start_range..].iter().max().copied();
        }

        None
    }
}

mod part_2 {
    use std::iter::once;

    use super::*;

    pub fn solve(banks: &[PowerBank]) -> u64 {
        banks
            .iter()
            .map(|bank| max_in_powerbank(bank))
            //     .inspect(|d| println!("{d}"))
            .sum()
    }

    fn max_in_powerbank(powerbank: &PowerBank) -> u64 {
        let digits = max_recursive(&[], 0, 12, &powerbank.banks);
        digits.iter().fold(0u64, |acc, &d| acc * 10 + d)
    }

    // Greedy idea to grab largest value but room left over in the array to get the full 12
    fn max_recursive(
        digits: &[u64],
        start_index: usize,
        remaining_digits: u32,
        bank: &[u64],
    ) -> Vec<u64> {
        if remaining_digits == 0 {
            return digits.to_vec();
        }

        // Ensure we have enough digits left after our choice. Constrained range search
        let search_end = bank.len() - remaining_digits as usize + 1;
        if let Some((idx, &max)) = bank[start_index..search_end].iter().enumerate().max_by(
            |(idx_a, val_a), (idx_b, val_b)| {
                // If there's a tie, pick lower index
                val_a.cmp(&val_b).then(idx_b.cmp(idx_a))
            },
        ) {
            let updated_digits = digits
                .iter()
                .copied()
                .chain(once(max))
                .collect::<Vec<u64>>();
            let actual_idx = start_index + idx + 1;
            return max_recursive(&updated_digits, actual_idx, remaining_digits - 1, bank);
        }

        vec![]
    }
}

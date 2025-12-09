mod gimme_input;

fn main() {
    let input = gimme_input::INPUT_FINAL;
    let machines = parse(input);
    let solved = part_2::solve(&machines);
    println!("{solved}");
}

/// (target_lights, buttons, joltage_targets)
pub type Machine = (u64, Vec<u64>, Vec<u64>);

pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_machine)
        .collect()
}

fn parse_machine(line: &str) -> Option<Machine> {
    // Parse target state from [.##.] format
    let target = line
        .split('[')
        .nth(1)?
        .split(']')
        .next()?
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0u64, |acc, (i, _)| acc | (1 << i));

    // Parse buttons from (0,2,3) format
    let buttons = line
        .split('(')
        .skip(1)
        .filter_map(|part| {
            part.split(')').next().map(|nums| {
                nums.split(',')
                    .filter_map(|n| n.trim().parse::<u32>().ok())
                    .fold(0u64, |acc, bit| acc | (1 << bit))
            })
        })
        .collect();

    // Parse joltage requirements from {3,5,4,7} format
    let joltage = line
        .split('{')
        .nth(1)?
        .split('}')
        .next()?
        .split(',')
        .filter_map(|n| n.trim().parse::<u64>().ok())
        .collect();

    Some((target, buttons, joltage))
}

pub mod part_1 {
    use super::Machine;
    use itertools::Itertools;

    pub fn solve(machines: &[Machine]) -> u64 {
        machines.iter().map(min_presses).sum()
    }

    /// "Pressing" is equiv to a XOR
    fn press(state: u64, button: u64) -> u64 {
        state ^ button
    }

    fn min_presses((target, buttons, _): &Machine) -> u64 {
        let sorted_buttons: Vec<_> = buttons
            .iter()
            .copied()
            .sorted_by_key(|b| b.count_ones())
            .collect();

        (0..=sorted_buttons.len())
            .find_map(|count| {
                sorted_buttons
                    .iter()
                    .combinations(count)
                    .find(|combo| {
                        let final_state = combo
                            .iter()
                            .fold(0u64, |state, &&button| press(state, button));
                        final_state == *target
                    })
                    .map(|_| count as u64)
            })
            .unwrap_or(0)
    }
}

pub mod part_2 {
    use z3::ast::Int;
    use z3::{Context, Optimize, SatResult};

    use super::Machine;

    pub fn solve(machines: &[Machine]) -> u64 {
        machines.iter().map(min_presses).sum()
    }

    fn min_presses((_, buttons, joltage): &Machine) -> u64 {
        let ctx = Context::thread_local();

        z3::with_z3_context(&ctx, || {
            let opt = Optimize::new();
            let num_counters = joltage.len();

            // Create a variable for each button that represents the number of times it was pressed
            let button_vars: Vec<Int> = (0..buttons.len())
                .map(|i| Int::fresh_const(&format!("b{i}")))
                .collect();

            for var in &button_vars {
                opt.assert(&var.ge(&Int::from_i64(0)));
            }

            for counter_idx in 0..num_counters {
                let target = Int::from_i64(joltage[counter_idx] as i64);

                // Collect all button presses that affect this counter
                let terms: Vec<&Int> = buttons
                    .iter()
                    .enumerate()
                    .filter(|&(_, &button_mask)| (button_mask >> counter_idx) & 1 == 1)
                    .map(|(i, _)| &button_vars[i])
                    .collect();

                if terms.is_empty() {
                    // No button affects this counter, it's unsolvable
                    if joltage[counter_idx] != 0 {
                        return 0;
                    }
                } else {
                    let sum = Int::add(&terms);
                    opt.assert(&sum.eq(&target));
                }
            }

            // Minimize total presses
            let all_refs: Vec<&Int> = button_vars.iter().collect();
            let total = Int::add(&all_refs);
            opt.minimize(&total);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().expect("Died on getting model");
                    let result = model.eval(&total, true).expect("Died on getting result");
                    result.as_i64().unwrap() as u64
                }
                _ => 0,
            }
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_parse() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        // First machine: [.##.], bits 1 and 2 are set
        assert_eq!(machines[0].0, 0b0110);
    }

    #[test]
    fn test_part1() {
        let machines = parse(EXAMPLE);
        assert_eq!(part_1::solve(&machines), 7);
    }

    #[test]
    fn test_part2() {
        let machines = parse(EXAMPLE);
        assert_eq!(part_2::solve(&machines), 33);
    }
}

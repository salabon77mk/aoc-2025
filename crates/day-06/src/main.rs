mod gimme_input;

fn main() {
    let nums = gimme_input::INPUT_FINAL_NUMS;
    let ops = gimme_input::INPUT_FINAL_OPS;

    let numops = part_2::parse(nums, ops);

    let total = part_2::solve(&numops);
    println!("{total}");
}

#[derive(Debug, Clone, Copy)]
enum Exp {
    Add,
    Mult,
}

mod part_1 {
    use super::*;

    pub fn solve(num_ops: &NumOps) -> u64 {
        num_ops
            .nums
            .iter()
            .zip(num_ops.ops.iter())
            .map(|(nums, op)| {
                let num_iter = nums.iter();

                match op {
                    Exp::Add => num_iter.sum::<u64>(),
                    Exp::Mult => num_iter.product::<u64>(),
                }
            })
            .sum::<u64>()
    }

    #[derive(Debug, Clone)]
    struct NumOps {
        pub nums: Vec<Vec<u64>>,
        pub ops: Vec<Exp>,
    }

    impl NumOps {
        fn new(nums: Vec<Vec<u64>>, ops: Vec<Exp>) -> Self {
            Self { nums, ops }
        }
    }

    fn parse(nums: &str, ops: &str) -> NumOps {
        let nums = nums
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|num_str| {
                num_str
                    .iter()
                    .map(|num| num.parse::<u64>().expect("Wack num"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let column_representation: Vec<Vec<u64>> = (0..nums[0].len())
            .map(|col_idx| nums.iter().map(|row| row[col_idx]).collect())
            .collect();

        let ops = ops
            .lines()
            .flat_map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|op| match op {
                "*" => Exp::Mult,
                "+" => Exp::Add,
                _ => unreachable!("Wack op"),
            })
            .collect::<Vec<_>>();

        NumOps::new(column_representation, ops)
    }
}

mod part_2 {
    use super::*;

    pub fn parse(nums: &str, ops: &str) -> Vec<(Vec<u64>, Exp)> {
        let lines: Vec<&str> = nums.lines().collect();
        // this is kind of a big assumption but it works lol
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let ops: Vec<Exp> = ops
            .split_whitespace()
            .map(|op| match op {
                "*" => Exp::Mult,
                "+" => Exp::Add,
                _ => unreachable!("Wack op"),
            })
            .collect();

        // read the input vertically as is inluding spaces!
        let columns: Vec<String> = (0..width)
            .map(|col| {
                lines
                    .iter()
                    .map(|line| line.chars().nth(col).unwrap_or(' '))
                    .collect()
            })
            .collect();

        // use empty columns as the divider between problems
        let problems: Vec<Vec<String>> = columns
            .split(|col: &String| col.trim().is_empty())
            .filter(|group| !group.is_empty())
            .map(|group| group.to_vec())
            .collect();

        // Convert each string based rep back to num rep vec and pair each vec with op
        problems
            .iter()
            .zip(ops.iter())
            .map(|(problem_cols, op)| {
                let nums = convert_problem(problem_cols);
                (nums, op.clone())
            })
            .collect()
    }

    fn convert_problem(columns: &[String]) -> Vec<u64> {
        columns
            .iter()
            .rev() // Read right-to-left
            .map(|col| {
                col.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<u64>()
                    .expect("Wack conversion")
            })
            .collect()
    }

    pub fn solve(num_ops: &[(Vec<u64>, Exp)]) -> u64 {
        num_ops
            .iter()
            .map(|(nums, op)| {
                let num_iter = nums.iter();

                match op {
                    Exp::Add => num_iter.sum::<u64>(),
                    Exp::Mult => num_iter.product::<u64>(),
                }
            })
            .inspect(|num| println!("{num}"))
            .sum::<u64>()
    }
}

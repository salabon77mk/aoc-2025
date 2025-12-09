use std::collections::HashMap;

mod gimme_input;

fn main() {
    let input = gimme_input::INPUT_FINAL;
    let graph = parse(input);
    let solved = part_2::solve(&graph);
    println!("{solved}");
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(name, outputs)| {
            (
                name.to_owned(),
                outputs.split_whitespace().map(String::from).collect(),
            )
        })
        .collect()
}

mod part_1 {
    use std::collections::HashMap;

    pub fn solve(graph: &HashMap<String, Vec<String>>) -> u64 {
        let mut memo = HashMap::new();
        count_paths("you", graph, &mut memo)
    }

    // I think it's fine to have an accumulator or something like a cache be mutable?
    fn count_paths(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        memo: &mut HashMap<String, u64>,
    ) -> u64 {
        if node == "out" {
            return 1;
        }
        if let Some(&cached) = memo.get(node) {
            return cached;
        }

        let count = graph
            .get(node)
            .into_iter()
            .flatten()
            .map(|child| count_paths(child, graph, memo))
            .sum();

        memo.insert(node.to_string(), count);
        count
    }
}

mod part_2 {
    use std::collections::HashMap;

    pub fn solve(graph: &HashMap<String, Vec<String>>) -> u64 {
        let mut memo = HashMap::new();
        count_paths("svr", false, false, graph, &mut memo)
    }

    fn count_paths(
        node: &str,
        seen_fft: bool,
        seen_dac: bool,
        graph: &HashMap<String, Vec<String>>,
        memo: &mut HashMap<(String, bool, bool), u64>,
    ) -> u64 {
        let seen_fft = seen_fft || node == "fft";
        let seen_dac = seen_dac || node == "dac";

        if node == "out" {
            return if seen_fft && seen_dac { 1 } else { 0 };
        }

        let key = (node.to_string(), seen_fft, seen_dac);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let count = graph
            .get(node)
            .into_iter()
            .flatten()
            .map(|child| count_paths(child, seen_fft, seen_dac, graph, memo))
            .sum();

        memo.insert(key, count);
        count
    }
}

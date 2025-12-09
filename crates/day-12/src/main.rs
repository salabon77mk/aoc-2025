mod gimme_input;

fn main() {
    let input = gimme_input::INPUT_FINAL;

    let solved = solve(input);
    println!("{solved}");
}

// in an effort to make a dumb heuristic to trim the inputs we def couldn't solve, we...solved it?
// I'm okay with that
fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let area: usize = parts[0]
                .trim_end_matches(':')
                .split('x')
                .map(|n| n.parse::<usize>().expect("wack input"))
                .product();
            let total: usize = parts[1..]
                .iter()
                .map(|n| n.parse::<usize>().expect("wack shape"))
                .sum();
            total * 7 <= area
        })
        .count()
}

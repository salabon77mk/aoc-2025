use std::collections::HashSet;

mod gimme_input;

fn main() {
    let input = gimme_input::INPUT_FINAL;
    let points = parse(input);

    let total = part_2::solve(&points);
    println!("{total}");
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| line.split(",").collect::<Vec<_>>())
        .map(|points| {
            let x = points[0].parse::<u64>().expect("Wack x value");
            let y = points[1].parse::<u64>().expect("Wack y val");
            let z = points[2].parse::<u64>().expect("Wack z value");
            Point(x, y, z)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(pub u64, pub u64, pub u64);

impl Point {
    fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.0.abs_diff(other.0);
        let dy = self.1.abs_diff(other.1);
        let dz = self.2.abs_diff(other.2);
        dx * dx + dy * dy + dz * dz
    }
}

fn distances(points: &[Point]) -> Vec<(u64, usize, Point, usize, Point)> {
    (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, points[i], j, points[j])))
        .map(|(i, point_i, j, point_j)| {
            (
                points[i].distance_squared(&points[j]),
                i,
                point_i,
                j,
                point_j,
            )
        })
        .collect::<Vec<_>>()
}
fn merge(circuits: Vec<HashSet<usize>>, a: usize, b: usize) -> Vec<HashSet<usize>> {
    let pos_a = circuits.iter().position(|s| s.contains(&a)).unwrap();
    let pos_b = circuits.iter().position(|s| s.contains(&b)).unwrap();

    if pos_a == pos_b {
        circuits // already same, no-op
    } else {
        let merged: HashSet<usize> = circuits[pos_a].union(&circuits[pos_b]).copied().collect();

        circuits
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| *idx != pos_a && *idx != pos_b)
            .map(|(_, s)| s)
            .chain(std::iter::once(merged))
            .collect()
    }
}
mod part_1 {
    use std::collections::HashSet;

    use crate::{Point, distances, merge};

    pub fn solve(points: &[Point], n: usize) -> u64 {
        let sorted_distance_pairs = {
            // can't sort without a mut here :(
            let mut distance_pairs = distances(points);
            distance_pairs.sort_by_key(|(dist, _, _, _, _)| *dist);
            distance_pairs
        };

        let initial: Vec<HashSet<usize>> = (0..points.len()).map(|i| HashSet::from([i])).collect();

        let circuits = sorted_distance_pairs
            .iter()
            .take(n)
            .fold(initial, |circuits, (_, a, _, b, _)| merge(circuits, *a, *b));

        let sizes: Vec<u64> = {
            let mut sizes: Vec<u64> = circuits.iter().map(|s| s.len() as u64).collect();
            sizes.sort();
            sizes
        };
        sizes.into_iter().rev().take(3).product()
    }
}

mod part_2 {
    use std::collections::HashSet;

    use crate::{Point, distances, merge};

    pub fn solve(points: &[Point]) -> u64 {
        let sorted_distance_pairs = {
            // can't sort without a mut here :(
            let mut distance_pairs = distances(points);
            distance_pairs.sort_by_key(|(dist, _, _, _, _)| *dist);
            distance_pairs
        };

        let mut circuits: Vec<HashSet<usize>> =
            (0..points.len()).map(|i| HashSet::from([i])).collect();

        for (_, idx_point_a, _, idx_point_b, _) in &sorted_distance_pairs {
            circuits = merge(circuits, *idx_point_a, *idx_point_b);
            if circuits.len() == 1 {
                return points[*idx_point_a].0 * points[*idx_point_b].0;
            }
        }
        unreachable!()
    }
}

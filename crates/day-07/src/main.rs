mod gimme_input;

use utils::grid::Grid;

fn main() {
    let input = gimme_input::INPUT_FINAL;
    let grid = parse(input);
    let splits = part_2::solve(&grid);
    println!("{splits}");
}

fn parse(input: &str) -> Grid<Manifold> {
    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Manifold::Empty,
                    'S' => Manifold::Start,
                    '|' => Manifold::Beam,
                    '^' => Manifold::Splitter,
                    _ => unreachable!("Wack input"),
                })
                .collect()
        })
        .collect();

    Grid::new(cells)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Manifold {
    Empty,
    Splitter,
    Beam,
    Start,
}

impl Manifold {
    fn to_char(self) -> char {
        match self {
            Manifold::Empty => '.',
            Manifold::Splitter => '^',
            Manifold::Beam => '|',
            Manifold::Start => 'S',
        }
    }
}

mod part_1 {
    use crate::Manifold;
    use std::collections::HashSet;
    use utils::grid::{Direction, Grid, Position};

    pub fn solve(grid: &Grid<Manifold>) -> u64 {
        let start = grid
            .positions()
            .find(|&pos| grid.get(pos) == Some(&Manifold::Start))
            .expect("No start found");

        let (split_count, _, beam_positions) =
            trace_beam(grid, start, HashSet::new(), HashSet::new());

        // Visualization
        let visual_grid = add_beam_markers(grid, &beam_positions);
        println!("\nVisualization:");
        print_grid(&visual_grid);

        split_count
    }

    fn trace_beam(
        grid: &Grid<Manifold>,
        pos: Position,
        visited_splitters: HashSet<Position>,
        beam_positions: HashSet<Position>,
    ) -> (u64, HashSet<Position>, HashSet<Position>) {
        // Collect beam path going south until we hit a splitter or exit
        let path: Vec<Position> = std::iter::successors(Some(pos), |&p| {
            let next = p.moved(Direction::South);
            grid.get(next).map(|_| next)
        })
        .take_while(|&p| grid.get(p) != Some(&Manifold::Splitter))
        .collect();

        // Add path to beam positions so we can visualize it later
        let beam_positions: HashSet<Position> = beam_positions
            .union(&path.iter().copied().collect())
            .copied()
            .collect();

        // the last position in the path MUST be the path as it's the invariant we're relying on above
        let last_pos = path.last().copied().unwrap_or(pos);
        let splitter_pos = last_pos.moved(Direction::South);

        match grid.get(splitter_pos) {
            Some(&Manifold::Splitter) if !visited_splitters.contains(&splitter_pos) => {
                // Mark this splitter as visited...once() and fold() are my functional best friends
                let visited_splitters: HashSet<Position> = visited_splitters
                    .union(&std::iter::once(splitter_pos).collect())
                    .copied()
                    .collect();

                let (total_splits, final_visited, final_beams) = Direction::left_right()
                    .into_iter()
                    .map(|dir| splitter_pos.moved(dir))
                    .filter(|&adj| grid.get(adj).is_some())
                    .fold(
                        (0, visited_splitters, beam_positions),
                        |(splits, visited, beams), next_pos| {
                            let (new_splits, new_visited, new_beams) =
                                trace_beam(grid, next_pos, visited, beams);
                            (splits + new_splits, new_visited, new_beams)
                        },
                    );

                (1 + total_splits, final_visited, final_beams)
            }
            _ => (0, visited_splitters, beam_positions),
        }
    }

    fn add_beam_markers(grid: &Grid<Manifold>, positions: &HashSet<Position>) -> Grid<Manifold> {
        let new_cells = grid
            .cells()
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        let pos = Position(x as i32, y as i32);
                        if *cell == Manifold::Empty && positions.contains(&pos) {
                            Manifold::Beam
                        } else {
                            *cell
                        }
                    })
                    .collect()
            })
            .collect();
        Grid::new(new_cells)
    }

    fn print_grid(grid: &Grid<Manifold>) {
        for row in grid.cells() {
            println!("{}", row.iter().map(|c| c.to_char()).collect::<String>());
        }
    }
}

mod part_2 {
    use crate::Manifold;
    use std::collections::HashMap;
    use utils::grid::{Direction, Grid, Position};

    pub fn solve(grid: &Grid<Manifold>) -> u64 {
        let start = grid
            .positions()
            .find(|&pos| grid.get(pos) == Some(&Manifold::Start))
            .expect("No start found");

        let (count, _) = count_timelines(grid, start, HashMap::new());
        count
    }

    fn count_timelines(
        grid: &Grid<Manifold>,
        pos: Position,
        cache: HashMap<Position, u64>,
    ) -> (u64, HashMap<Position, u64>) {
        if let Some(&count) = cache.get(&pos) {
            return (count, cache);
        }

        // find the next splitter, similar idea to above
        let splitter_pos = std::iter::successors(Some(pos), |&p| {
            let next = p.moved(Direction::South);
            grid.get(next).map(|_| next)
        })
        .skip(1)
        .find(|&p| grid.get(p) == Some(&Manifold::Splitter));

        let (count, cache) = match splitter_pos {
            Some(splitter) => Direction::left_right()
                .into_iter()
                .map(|dir| splitter.moved(dir))
                .filter(|&next_pos| grid.get(next_pos).is_some())
                .fold((0, cache), |(sum, cache), next_pos| {
                    let (count, cache) = count_timelines(grid, next_pos, cache);
                    (sum + count, cache)
                }),
            None => (1, cache),
        };

        // yea this is kind of ridiculous but we gotta try to live with the spirit of the challenge.
        // otherwise could been passing along a mut hashmap cache through the recursion
        let cache: HashMap<Position, u64> = cache
            .into_iter()
            .chain(std::iter::once((pos, count)))
            .collect();
        (count, cache)
    }
}

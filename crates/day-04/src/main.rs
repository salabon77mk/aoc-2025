use std::time::Instant;

use crate::grid::Grid;

mod gimme_input;
mod grid;

fn main() {
    let input = gimme_input::INPUT_FINAL;

    let grid = parse(input);

    let start = Instant::now();

    let accessible = part_2::solve(&grid);

    let solve_time = start.elapsed();

    println!("{accessible}");
    println!("Solved after {solve_time:?}");
}

fn parse(input: &str) -> Grid<PrintingDepartment> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => PrintingDepartment::Paper,
                    '.' => PrintingDepartment::Empty,
                    _ => unreachable!("Weird input"),
                })
                .collect()
        })
        .collect();

    Grid::new(grid)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum PrintingDepartment {
    Paper,
    Empty,
}

mod part_1 {
    use crate::grid::{Direction, Position};

    use super::*;

    pub fn solve(grid: &Grid<PrintingDepartment>) -> u64 {
        grid.positions()
            // work only with Paper
            .filter(|&pos| grid.get(pos) == Some(&PrintingDepartment::Paper))
            // compute which papers are accessible
            .filter_map(|pos| can_access(grid, pos))
            .filter(|&access| access)
            .count() as u64
    }

    pub fn can_access(grid: &Grid<PrintingDepartment>, pos: Position) -> Option<bool> {
        // Interesting design question here, do we make the caller pass in only valid Paper positions?
        // let Some(piece) = grid.get(pos) else {
        //     return vec![];
        // };

        // if *piece == PrintingDepartment::Empty {
        //     return vec![];
        // }

        let rolls = Direction::all()
            .iter()
            // get the next position and check if it's Paper
            .filter_map(|dir| grid.get(pos.moved(*dir)))
            .filter(|dep| **dep == PrintingDepartment::Paper)
            .count();

        Some(rolls < 4)
    }
}

mod part_2 {
    use crate::{
        PrintingDepartment,
        grid::{Grid, Position},
    };

    pub fn solve(grid: &Grid<PrintingDepartment>) -> u64 {
        solve_recursive(grid, 0)
    }

    fn solve_recursive(grid: &Grid<PrintingDepartment>, accumulated: u64) -> u64 {
        let accessible = removable_rolls(grid);

        if accessible.is_empty() {
            accumulated
        } else {
            let new_grid = remove_rolls(grid, &accessible);
            solve_recursive(&new_grid, accumulated + accessible.len() as u64)
        }
    }

    // gimme the rolls that can be removed
    fn removable_rolls(grid: &Grid<PrintingDepartment>) -> Vec<Position> {
        grid.positions()
            .filter(|&pos| grid.get(pos) == Some(&PrintingDepartment::Paper))
            .filter(|&pos| super::part_1::can_access(grid, pos) == Some(true))
            .collect()
    }

    // IT'S FUNCTIONAL BABY. Anyway this does let us make snapshots of each grid through the process which is rad
    fn remove_rolls(
        grid: &Grid<PrintingDepartment>,
        positions: &[Position],
    ) -> Grid<PrintingDepartment> {
        let new_cells = grid
            .cells()
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        if positions.contains(&Position(x as i32, y as i32)) {
                            PrintingDepartment::Empty
                        } else {
                            cell.clone()
                        }
                    })
                    .collect()
            })
            .collect();

        Grid::new(new_cells)
    }
}

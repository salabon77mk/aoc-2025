#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        let width = cells.first().map(|row| row.len()).unwrap_or(0);
        let height = cells.len();

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        let Position(x, y) = pos;
        if x >= 0 && y >= 0 {
            self.cells.get(y as usize)?.get(x as usize)
        } else {
            None
        }
    }

    pub fn cells(&self) -> &Vec<Vec<T>> {
        &self.cells
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Position(x as i32, y as i32)))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        use Direction::*;
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }

    pub fn left_right() -> [Direction; 2] {
        use Direction::{East, West};
        [West, East]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn move_in(pos: Position, dir: Direction) -> Self {
        match dir {
            Direction::North => Position(pos.0, pos.1 - 1),
            Direction::NorthEast => Position(pos.0 + 1, pos.1 - 1),
            Direction::East => Position(pos.0 + 1, pos.1),
            Direction::SouthEast => Position(pos.0 + 1, pos.1 + 1),
            Direction::South => Position(pos.0, pos.1 + 1),
            Direction::SouthWest => Position(pos.0 - 1, pos.1 + 1),
            Direction::West => Position(pos.0 - 1, pos.1),
            Direction::NorthWest => Position(pos.0 - 1, pos.1 - 1),
        }
    }

    pub fn move_with_bounds_check(pos: Position, dir: Direction) -> Option<Position> {
        let Position(x, y) = pos;
        match dir {
            Direction::North if y > 0 => Some(Position(x, y - 1)),
            Direction::NorthEast if y > 0 => Some(Position(x + 1, y - 1)),
            Direction::East => Some(Position(x + 1, y)),
            Direction::SouthEast => Some(Position(x + 1, y + 1)),
            Direction::South => Some(Position(x, y + 1)),
            Direction::SouthWest if x > 0 => Some(Position(x - 1, y + 1)),
            Direction::West if x > 0 => Some(Position(x - 1, y)),
            Direction::NorthWest if x > 0 && y > 0 => Some(Position(x - 1, y - 1)),
            _ => None,
        }
    }

    pub fn moved(self, dir: Direction) -> Self {
        Position::move_in(self, dir)
    }
}

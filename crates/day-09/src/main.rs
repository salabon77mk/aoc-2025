use std::time::Instant;

mod gimme_input;

fn main() {
    let input = gimme_input::INPUT_FINAL;
    let points = parse(input);

    let start = Instant::now();
    let solved = part_2::solve(&points);
    let end = start.elapsed();

    println!("{solved} in {end:?}");
}

#[derive(Copy, Clone, Debug)]
struct Point(pub i64, pub i64);

fn area(Point(x1, y1): Point, Point(x2, y2): Point) -> i64 {
    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
}

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| line.split(",").collect::<Vec<_>>())
        .map(|coords| {
            let x = coords[0].parse::<i64>().expect("Wack x value");
            let y = coords[1].parse::<i64>().expect("Wack y value");
            Point(x, y)
        })
        .collect()
}

mod part_1 {
    use crate::{Point, area};

    pub fn solve(points: &[Point]) -> i64 {
        (0..points.len())
            .flat_map(|i| (i + 1..points.len()).map(move |j| (i, points[i], j, points[j])))
            .map(|(_, point_i, _, point_j)| area(point_i, point_j))
            .max()
            .expect("How did we not get a max")
    }
}

mod part_2 {
    use crate::{Point, area};

    use geo::{Contains, Coord, LineString, Polygon, Rect};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    pub fn solve(points: &[Point]) -> i64 {
        let coords: Vec<Coord<f64>> = points
            .iter()
            .map(|Point(x, y)| Coord {
                x: *x as f64,
                y: *y as f64,
            })
            .collect();

        let polygon = Polygon::new(LineString::from(coords), vec![]);

        (0..points.len())
            .into_par_iter()
            .flat_map_iter(|i| (i + 1..points.len()).map(move |j| (points[i], points[j])))
            .filter(|(p1, p2)| {
                let rect = Rect::new(
                    Coord {
                        x: p1.0 as f64,
                        y: p1.1 as f64,
                    },
                    Coord {
                        x: p2.0 as f64,
                        y: p2.1 as f64,
                    },
                );
                polygon.contains(&rect)
            })
            .map(|(p1, p2)| area(p1, p2))
            .max()
            .expect("Wtf part 2")
    }
}

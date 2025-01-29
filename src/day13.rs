use crate::utils::read_lines;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

pub struct AocDay13 {
    points: Vec<(Point, Point, Point)>,
}

impl AocDay13 {
    pub fn new() -> Self {
        let lines = read_lines("input/day13.txt");
        let mut points = Vec::new();

        for i in (0..lines.len()).step_by(4) {
            let (a, b, c) = Self::read_points(&lines, i);
            points.push((a, b, c));
        }

        AocDay13 { points }
    }

    pub fn part1(&self) -> usize {
        Self::calc_total_price(&self.points)
    }

    pub fn part2(&self) -> usize {
        let new_points = self
            .points
            .iter()
            .map(|(a, b, c)| {
                (
                    *a,
                    *b,
                    Point::new(c.x + 10000000000000, c.y + 10000000000000),
                )
            })
            .collect::<Vec<_>>();

        Self::calc_total_price(&new_points)
    }

    fn calc_total_price(points: &Vec<(Point, Point, Point)>) -> usize {
        let mut total_price = 0;

        for (a, b, c) in points {
            let result = Self::solve(&a, &b, &c);
            total_price += result.0 * 3 + result.1;
        }

        total_price
    }

    fn solve(a: &Point, b: &Point, c: &Point) -> (usize, usize) {
        let a1 = a.x as f64;
        let b1 = b.x as f64;
        let c1 = c.x as f64;
        let a2 = a.y as f64;
        let b2 = b.y as f64;
        let c2 = c.y as f64;

        let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
        let y = (a1 * c2 - a2 * c1) / (a1 * b2 - a2 * b1);

        if x.fract() != 0.0 || y.fract() != 0.0 {
            return (0, 0);
        }

        (x as usize, y as usize)
    }

    fn read_points(lines: &Vec<String>, index: usize) -> (Point, Point, Point) {
        let a = lines[index].split(": ").collect::<Vec<_>>()[1];
        let b = lines[index + 1].split(": ").collect::<Vec<_>>()[1];
        let c = lines[index + 2].split(": ").collect::<Vec<_>>()[1];

        let a = a.split(", ").collect::<Vec<_>>();
        let b = b.split(", ").collect::<Vec<_>>();
        let c = c.split(", ").collect::<Vec<_>>();

        let (ax, ay) = (
            a[0][2..].parse::<usize>().unwrap(),
            a[1][2..].parse::<usize>().unwrap(),
        );
        let (bx, by) = (
            b[0][2..].parse::<usize>().unwrap(),
            b[1][2..].parse::<usize>().unwrap(),
        );
        let (cx, cy) = (
            c[0][2..].parse::<usize>().unwrap(),
            c[1][2..].parse::<usize>().unwrap(),
        );

        (Point::new(ax, ay), Point::new(bx, by), Point::new(cx, cy))
    }
}

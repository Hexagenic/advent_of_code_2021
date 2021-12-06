#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn parse_line(line: &str) -> Self {
        let parts: Vec<i64> = line
            .split(" -> ")
            .flat_map(|p| p.split(',').flat_map(str::parse::<i64>))
            .collect();

        Self::make(parts[0], parts[1], parts[2], parts[3])
    }

    fn make(x1: i64, y1: i64, x2: i64, y2: i64) -> Line {
        Line {
            from: Point { x: x1, y: y1 },
            to: Point { x: x2, y: y2 },
        }
    }

    fn is_hor_or_vert(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn get_points(&self) -> Vec<Point> {
        if self.from == self.to {
            return vec![self.from];
        } else if self.from.x != self.to.x {
            let x_dir = i64::signum(self.to.x - self.from.x);
            let y_dir = i64::signum(self.to.y - self.from.y);
            let x_dist = i64::abs(self.to.x - self.from.x);

            return (0..=x_dist)
                .map(|i| Point {
                    x: self.from.x + i * x_dir,
                    y: self.from.y + i * y_dir,
                })
                .collect();
        } else if self.from.y != self.to.y {
            let x_dir = i64::signum(self.to.x - self.from.x);
            let y_dir = i64::signum(self.to.y - self.from.y);
            let y_dist = i64::abs(self.to.y - self.from.y);
            return (0..=y_dist)
                .map(|i| Point {
                    x: self.from.x + i * x_dir,
                    y: self.from.y + i * y_dir,
                })
                .collect();
        }

        vec![]
    }

    fn overlap(a: &Line, b: &Line) -> Vec<Point> {
        if a == b {
            return vec![];
        }

        let a_left = i64::min(a.from.x, a.to.x);
        let b_left = i64::min(b.from.x, b.to.x);
        let a_right = i64::max(a.from.x, a.to.x);
        let b_right = i64::max(b.from.x, b.to.x);
        let a_top = i64::min(a.from.y, a.to.y);
        let b_top = i64::min(b.from.y, b.to.y);
        let a_bottom = i64::max(a.from.y, a.to.y);
        let b_bottom = i64::max(b.from.y, b.to.y);

        if b_left > a_right || b_right < a_left || b_top > a_bottom || b_bottom < a_top {
            return vec![];
        }

        let a_points = a.get_points();
        let b_points = b.get_points();

        return a_points
            .iter()
            .filter(|a| b_points.contains(a))
            .copied()
            .collect();
    }
}

pub fn part_a(file: &str) -> i64 {
    use rayon::prelude::*;

    let lines = file
        .lines()
        .map(Line::parse_line)
        .filter(Line::is_hor_or_vert)
        .collect::<Vec<_>>();

    let overlaps: std::collections::HashSet<Point> = lines
        .par_iter()
        .flat_map(|a| {
            lines
                .iter()
                .flat_map(move |b| Line::overlap(&a, &b))
                .collect::<Vec<Point>>()
        })
        .collect();

    overlaps.len() as i64
}

pub fn part_b(file: &str) -> i64 {
    use rayon::prelude::*;
    let lines = file.lines().map(Line::parse_line).collect::<Vec<_>>();

    let overlaps: std::collections::HashSet<Point> = lines
        .par_iter()
        .flat_map(|a| {
            lines
                .iter()
                .flat_map(move |b| Line::overlap(&a, &b))
                .collect::<Vec<Point>>()
        })
        .collect();

    overlaps.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day5_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(5, part_a(&file));
    }

    #[test]
    fn day5_part2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day5_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(12, part_b(&file));
    }

    #[test]
    fn line_to_points() {
        assert_eq!(
            Line::make(0, 0, 1, 0).get_points(),
            vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }]
        );
        assert_eq!(
            Line::make(0, 0, 0, 1).get_points(),
            vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }]
        );
        assert_eq!(
            Line::make(0, 0, 1, 1).get_points(),
            vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]
        );
        assert_eq!(
            Line::make(0, 0, 3, 3).get_points(),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 }
            ]
        );
        assert_eq!(
            Line::make(3, 3, 0, 0).get_points(),
            vec![
                Point { x: 3, y: 3 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 }
            ]
        );
        assert_eq!(
            Line::make(1, 0, 0, 0).get_points(),
            vec![Point { x: 1, y: 0 }, Point { x: 0, y: 0 },]
        );
        assert_eq!(
            Line::make(0, 1, 0, 0).get_points(),
            vec![Point { x: 0, y: 1 }, Point { x: 0, y: 0 },]
        );
        assert_eq!(
            Line::make(1, 2, 1, 1).get_points(),
            vec![Point { x: 1, y: 2 }, Point { x: 1, y: 1 },]
        );
    }

    #[test]
    fn line_overlaps() {
        println!("Wut {:?}", Line::make(0, 1, 2, 1).get_points());
        println!("Wut {:?}", Line::make(1, 0, 1, 2).get_points());
        assert_eq!(
            Line::overlap(&Line::make(0, 1, 2, 1), &Line::make(1, 0, 1, 2)),
            vec![Point { x: 1, y: 1 }]
        );
    }
}

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
}

struct Canvas {
    pixels: Vec<u8>
}

impl Canvas {
    fn new() -> Self {
        Canvas { pixels: vec![0; 1000 * 1000]}
    }

    fn draw(self, line: &Line) -> Self {
        let mut pixels = self.pixels;
        for p in line.get_points() {
            pixels[(p.x * 1000 + p.y) as usize] += 1;
        }

        Canvas {pixels}
    }

    fn count(&self) -> i64 {
        self.pixels.iter().filter(|v| v >= &&2).count() as i64
    }
}

pub fn part_a(file: &str) -> i64 {
    file
        .lines()
        .map(Line::parse_line)
        .filter(Line::is_hor_or_vert)
        .fold(Canvas::new(), |acc, l| acc.draw(&l)).count()
}

pub fn part_b(file: &str) -> i64 {
    file
        .lines()
        .map(Line::parse_line)
        .fold(Canvas::new(), |acc, l| acc.draw(&l)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_solution() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day5.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(7142, part_a(&file));
    }

    #[test]
    fn part2_solution() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day5.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(20012, part_b(&file));
    }

    #[test]
    fn part1_example() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day5_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(5, part_a(&file));
    }

    #[test]
    fn part2_example() {
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
}

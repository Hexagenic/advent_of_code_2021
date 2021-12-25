use crate::solutions::Solution;

#[derive(Debug)]
struct Probe {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
    max_height: i64,
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

#[derive(Eq, PartialEq)]
enum SquareCheck {
    Over,
    In,
    Under,
}

impl Square {
    fn test_point(&self, pos_x: i64, pos_y: i64) -> SquareCheck {
        if pos_y < self.y1 {
            SquareCheck::Under
        } else if pos_y <= self.y2 && pos_x >= self.x1 && pos_x <= self.x2 {
            SquareCheck::In
        } else {
            SquareCheck::Over
        }
    }

    fn parse(line: &str) -> Square {
        let re = regex::Regex::new(r"^target area: x=(\d*)..(\d*), y=(-?\d*)..(-?\d*)$").unwrap();

        let numbers = re.captures_iter(line).next().unwrap();

        Square {
            x1: numbers[1].parse().unwrap(),
            x2: numbers[2].parse().unwrap(),
            y1: numbers[3].parse().unwrap(),
            y2: numbers[4].parse().unwrap(),
        }
    }
}

impl Probe {
    fn launch(vel_x: i64, vel_y: i64) -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
            max_height: 0,
        }
    }

    fn step(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        self.vel_x -= self.vel_x.signum();
        self.vel_y -= 1;
        self.max_height = self.max_height.max(self.pos_y);
    }

    fn step_until_square(&mut self, square: &Square) -> bool {
        loop {
            self.step();

            let result = square.test_point(self.pos_x, self.pos_y);

            if result == SquareCheck::In {
                return true;
            } else if result == SquareCheck::Under {
                return false;
            }
        }
    }

    #[allow(dead_code)]
    fn hits_square(vel_x: i64, vel_y: i64, square: &Square) -> bool {
        Self::launch(vel_x, vel_y).step_until_square(square)
    }

    fn find_highest_y(square: &Square) -> i64 {
        let mut highest = 0;
        for y_vel in square.y1..-square.y1 {
            for x_vel in 0..square.x2 {
                let mut probe = Probe::launch(x_vel, y_vel);
                if probe.step_until_square(square) {
                    highest = highest.max(probe.max_height)
                }
            }
        }

        highest
    }

    fn count_hits(square: &Square) -> i64 {
        let mut hits = 0;
        for y_vel in square.y1..-square.y1 {
            for x_vel in 0..(square.x2 * 2) {
                let mut probe = Probe::launch(x_vel, y_vel);
                if probe.step_until_square(square) {
                    hits += 1
                }
            }
        }

        hits
    }
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(Probe::find_highest_y(&Square::parse(file)))
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(Probe::count_hits(&Square::parse(file)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let square = Square {
            x1: 20,
            x2: 30,
            y1: -10,
            y2: -5,
        };
        assert_eq!(true, Probe::hits_square(7, 2, &square));
        assert_eq!(true, Probe::hits_square(6, 3, &square));
        assert_eq!(true, Probe::hits_square(9, 0, &square));
        assert_eq!(false, Probe::hits_square(17, -4, &square));

        let mut probe = Probe::launch(6, 9);
        probe.step_until_square(&square);
        assert_eq!(45, probe.max_height);

        assert_eq!(square, Square::parse("target area: x=20..30, y=-10..-5"));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::Integer(112),
            part_b("target area: x=20..30, y=-10..-5")
        );
    }

    #[test]
    fn solution_1() {
        assert_eq!(
            Solution::Integer(3655),
            part_a("target area: x=209..238, y=-86..-59")
        );
    }
}

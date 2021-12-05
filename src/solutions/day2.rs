use crate::error::Error;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_command(input: &str) -> Result<Command, Error> {
    let parts: Vec<&str> = input.split(' ').collect();
    let length: i64 = parts[1].parse()?;
    let command = parts[0];

    match command {
        "forward" => Ok(Command::Forward(length)),
        "down" => Ok(Command::Down(length)),
        "up" => Ok(Command::Up(length)),
        _ => Err(Error::IoError(std::io::Error::from(
            std::io::ErrorKind::InvalidData,
        ))),
    }
}

#[cfg(not(feature = "with-rayon"))]
pub fn part_a(file: &str) -> i64 {
    let commands = file
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(parse_command);

    let mut depth: i64 = 0;
    let mut distance: i64 = 0;

    for command in commands {
        match command {
            Command::Forward(d) => distance += d,
            Command::Down(d) => depth += d,
            Command::Up(d) => depth -= d,
        }
    }

    depth * distance
}

#[cfg(feature = "with-rayon")]
pub fn part_a(file: &str) -> i64 {
    use rayon::prelude::*;

    struct Location {
        depth: i64,
        distance: i64,
    }

    impl Default for Location {
        fn default() -> Self {
            Location {
                depth: 0,
                distance: 0,
            }
        }
    }

    impl Location {
        fn from_command(c: Command) -> Self {
            match c {
                Command::Forward(d) => Location {
                    depth: 0,
                    distance: d,
                },
                Command::Down(d) => Location {
                    depth: d,
                    distance: 0,
                },
                Command::Up(d) => Location {
                    depth: -d,
                    distance: 0,
                },
            }
        }
        fn add(a: Location, b: Location) -> Self {
            Location {
                depth: a.depth + b.depth,
                distance: a.distance + b.distance,
            }
        }
    }

    let location: Location = file
        .par_lines()
        .flat_map(parse_command)
        .map(Location::from_command)
        .reduce(|| Location::default(), Location::add);
    location.depth * location.distance
}

pub fn part_b(file: &str) -> i64 {
    let commands = file
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(parse_command);

    let mut depth: i64 = 0;
    let mut distance: i64 = 0;
    let mut aim: i64 = 0;

    for command in commands {
        match command {
            Command::Forward(d) => {
                distance += d;
                depth += d * aim;
            }
            Command::Down(d) => aim += d,
            Command::Up(d) => aim -= d,
        }
    }

    depth * distance
}

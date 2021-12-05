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

pub fn part_a() -> Result<i64, Error> {
    let path = std::env::current_dir()?.join("input").join("day2.txt");

    let file = std::fs::read_to_string(path)?;
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

    Ok(depth * distance)
}

pub fn part_b() -> Result<i64, Error> {
    let path = std::env::current_dir()?.join("input").join("day2.txt");

    let file = std::fs::read_to_string(path)?;
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

    Ok(depth * distance)
}

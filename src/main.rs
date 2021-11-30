use std::env;
mod solutions;
mod error;
use error::Error;

fn run() -> Result<(), Error> {
    let (times, puzzles) = get_args()?;

    for _ in 0..times {
        println!("Solving");

        for day in &puzzles {
            let (part1, part2) = solutions::get_solution(*day);

            if let Some(part1) = part1 {
                println!("{}a: {}", day, part1());
            } else {
                println!("{}a: Incomplete", day);
            }

            if let Some(part2) = part2 {
                println!("{}b: {}", day, part2());
            } else {
                println!("{}b: Incomplete", day);
            }
        }

        println!("Done");
    }

    Ok(())
}

fn get_args() -> Result<(u32, Vec<u8>), Error> {
    let mut args = env::args().skip(1);
    let times: Option<Result<u32, _>> = args.next().map(|s| s.parse());

    if times.is_none() {
        return Ok((1, (1..=25).collect()));
    }

    let times = times.unwrap()?;

    let puzzles = args.map(|s| s.parse()).collect::<Result<Vec<u8>, _>>()?;

    if puzzles.is_empty() {
        return Ok((times, (1..=25).collect()));
    }

    Ok((times, puzzles))
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
        _ => (),
    }
}

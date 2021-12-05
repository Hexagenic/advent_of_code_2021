use std::env;
mod error;
mod solutions;
use error::Error;

fn run() -> Result<(), Error> {
    let (times, puzzles) = get_args()?;

    for _ in 0..times {
        println!("Solving");

        for day in &puzzles {
            let (solution1, solution2) = solutions::get_solution(*day);
            let input_file = format!("day{}.txt", day);
            let path = std::env::current_dir()
                .unwrap()
                .join("input")
                .join(&input_file);

            if let Some(solution1) = solution1 {
                let file: String = std::fs::read_to_string(&path)?;
                println!("{}a: {}", day, solution1(&file));
            } else {
                println!("{}a: Incomplete", day);
            }

            if let Some(solution2) = solution2 {
                let file: String = std::fs::read_to_string(&path)?;
                println!("{}a: {}", day, solution2(&file));
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
        return Ok((1, (1..=solutions::MAX_SOLVED_DAY).collect()));
    }

    let times = times.unwrap()?;

    let puzzles = args.map(|s| s.parse()).collect::<Result<Vec<u8>, _>>()?;

    if puzzles.is_empty() {
        return Ok((times, (1..=solutions::MAX_SOLVED_DAY).collect()));
    }

    Ok((times, puzzles))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}

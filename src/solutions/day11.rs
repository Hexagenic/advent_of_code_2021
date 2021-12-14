
type Octopi = Vec<u32>;

fn parse_octopi(input: &str) -> Octopi {
    input.chars().flat_map(|c| char::to_digit(c, 10)).collect()
}

fn step(previous: &Octopi) -> (Octopi, i64) {
    let current = previous.clone();
    let mut flashes = 0;

    

    (current, flashes)
}

fn step_n(initial: Octopi, steps: u32) -> i64 {

    let mut octopi = initial;
    let mut flashes: i64 = 0;
    for i in 0..steps {
        let step_result = step(&octopi);
        octopi = step_result.0;
        flashes += step_result.1;

    }

    flashes
}


pub fn part_a(file: &str) -> i64 {
    step_n(parse_octopi(file), 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_example() {
        let initial = parse_octopi("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526");
        assert_eq!(204, step_n(initial.clone(), 10));
        assert_eq!(1656, step_n(initial.clone(), 100));
    }
}
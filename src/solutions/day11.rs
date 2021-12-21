type Octopi = Vec<i32>;
type OctopiSlice = [i32];

#[allow(clippy::cast_possible_wrap)]
fn parse_octopi(input: &str) -> Octopi {
    input
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .map(|v| v as i32)
        .collect()
}

#[allow(clippy::cast_sign_loss)]
fn flash(octopi: &mut Octopi, x: i32, y: i32) -> i64 {
    let i_start = (y * 10 + x) as usize;

    if octopi[i_start] <= 9 {
        return 0;
    }

    octopi[i_start] = -1;

    let mut flashes = 1;
    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            if x_diff == 0 && y_diff == 0 {
                continue;
            }

            let n_x = x + x_diff;
            let n_y = y + y_diff;

            if (0..10).contains(&n_x) && (0..10).contains(&n_y) {
                let i = (n_y * 10 + n_x) as usize;

                if octopi[i] == -1 {
                    continue;
                }

                octopi[i] += 1;

                flashes += flash(octopi, n_x, n_y);
            }
        }
    }

    flashes
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
fn step(previous: &OctopiSlice) -> (Octopi, i64) {
    let mut current = previous.to_owned();
    let mut flashes = 0;

    for i in 0..current.len() {
        if current[i] == -1 {
            continue;
        }

        current[i] += 1;

        let x = (i % 10) as i32;
        let y = (i / 10) as i32;

        flashes += flash(&mut current, x, y);
    }

    for c in &mut current {
        if *c == -1 {
            *c = 0;
        }
    }

    (current, flashes)
}

fn step_n(initial: Octopi, steps: u32) -> i64 {
    let mut octopi = initial;
    let mut flashes: i64 = 0;
    for _ in 0..steps {
        let step_result = step(&octopi);
        octopi = step_result.0;
        flashes += step_result.1;
    }

    flashes
}

fn step_to_all_flash(initial: Octopi) -> i64 {
    let mut octopi = initial;
    let mut steps_taken: i64 = 0;

    loop {
        let step_result = step(&octopi);
        steps_taken += 1;
        if step_result.1 == octopi.len() as i64 {
            return steps_taken;
        }
        octopi = step_result.0;
    }
}

pub fn part_a(file: &str) -> i64 {
    step_n(parse_octopi(file), 100)
}

pub fn part_b(file: &str) -> i64 {
    step_to_all_flash(parse_octopi(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_STR: &str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

    #[test]
    fn part_a_example() {
        let initial = parse_octopi(INITIAL_STR);
        assert_eq!(35, step_n(initial.clone(), 2));
        assert_eq!(204, step_n(initial.clone(), 10));
        assert_eq!(1656, step_n(initial.clone(), 100));
    }

    #[test]
    fn part_b_example() {
        let initial = parse_octopi(INITIAL_STR);
        assert_eq!(195, step_to_all_flash(initial));
    }
}

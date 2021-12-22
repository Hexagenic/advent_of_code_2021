use crate::solutions::Solution;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn parse_fold(line: &str) -> Fold {
    let dir = line.chars().nth(11).unwrap();

    let num: i32 = (&line[13..]).parse().unwrap();

    if dir == 'x' {
        Fold::X(num)
    } else {
        Fold::Y(num)
    }
}

fn parse_dot(line: &str) -> (i32, i32) {
    let parts = line.split(',').flat_map(str::parse).collect::<Vec<_>>();

    (parts[0], parts[1])
}

fn fold_dot(dot: (i32, i32), fold: &Fold) -> (i32, i32) {
    match fold {
        Fold::X(num) => (
            if dot.0 > *num {
                num - (dot.0 - num)
            } else {
                dot.0
            },
            dot.1,
        ),
        Fold::Y(num) => (
            dot.0,
            if dot.1 > *num {
                num - (dot.1 - num)
            } else {
                dot.1
            },
        ),
    }
}

pub fn part_a(file: &str) -> Solution {
    let mut dots: Vec<(i32, i32)> = vec![];
    let mut folds: Vec<Fold> = vec![];

    let mut is_parsing_nunbers = true;

    for line in file.lines() {
        if line.is_empty() {
            is_parsing_nunbers = false;
            continue;
        }

        if is_parsing_nunbers {
            dots.push(parse_dot(line));
        } else {
            folds.push(parse_fold(line));
        }
    }

    let folded_dots = dots
        .iter()
        .map(|d| fold_dot(*d, &folds[0]))
        .collect::<std::collections::HashSet<_>>();

    Solution::Integer(folded_dots.len() as i64)
}

pub fn part_b(file: &str) -> Solution {
    let mut dots: Vec<(i32, i32)> = vec![];
    let mut folds: Vec<Fold> = vec![];

    let mut is_parsing_nunbers = true;

    for line in file.lines() {
        if line.is_empty() {
            is_parsing_nunbers = false;
            continue;
        }

        if is_parsing_nunbers {
            dots.push(parse_dot(line));
        } else {
            folds.push(parse_fold(line));
        }
    }

    let mut folded_dots: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();

    for mut dot in dots {
        for fold in &folds {
            dot = fold_dot(dot, fold);
        }
        folded_dots.insert(dot);
    }

    let width = folded_dots.iter().map(|d| d.0).max().unwrap();
    let height = folded_dots.iter().map(|d| d.1).max().unwrap();

    let mut output: String = "".to_string();
    for y in 0..=height {
        for x in 0..=width {
            if folded_dots.contains(&(x, y)) {
                output += "\u{2588}";
            } else {
                output += " ";
            }
        }
        if y != height {
            output += "\n";
        }
    }

    Solution::String(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day13_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(Solution::Integer(17), part_a(&file));
    }

    #[test]
    fn example_2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day13_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        let expected = "█████\n█   █\n█   █\n█   █\n█████";

        assert_eq!(Solution::String(expected.to_string()), part_b(&file));
    }
}

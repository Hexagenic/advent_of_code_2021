use crate::solutions::Solution;
use rayon::prelude::*;

pub fn part_a(file: &str) -> Solution {
    fn count_easy(acc: i64, output: &str) -> i64 {
        acc + (output
            .split_ascii_whitespace()
            .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
            .count() as i64)
    }

    Solution::Integer(
        file.lines()
            .filter_map(|l| l.split(" | ").nth(1))
            .fold(0, count_easy),
    )
}

fn to_segments(string: &str) -> u8 {
    string
        .chars()
        .fold(0, |acc, c| acc | 1 << ((c as u8) - b'a'))
}

fn make_key(input: &str) -> [u8; 10] {
    let inputs: Vec<&str> = input.split_ascii_whitespace().collect();

    let mut key: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0b111_1111, 0];

    // First fill in the obvious ones
    for i in &inputs {
        match i.len() {
            2 => key[1] = to_segments(i),
            3 => key[7] = to_segments(i),
            4 => key[4] = to_segments(i),
            _ => (),
        }
    }

    // Used to differentiate 2 and 5
    let segments_aeg = key[8] ^ key[4];

    for i in &inputs {
        match i.len() {
            6 => {
                let segments = to_segments(i);
                if segments & key[4] == key[4] {
                    key[9] = segments;
                } else if segments & key[7] == key[7] {
                    key[0] = segments;
                } else {
                    key[6] = segments;
                }
            }
            5 => {
                let segments = to_segments(i);
                if segments & key[7] == key[7] {
                    key[3] = segments;
                } else if segments & segments_aeg == segments_aeg {
                    key[2] = segments;
                } else {
                    key[5] = segments;
                }
            }
            _ => (),
        }
    }
    key
}

fn decode(display: &str, key: [u8; 10]) -> i64 {
    display.split_ascii_whitespace().fold(0, |acc, v| {
        acc * 10 + (key.iter().position(|w| *w == to_segments(v)).unwrap() as i64)
    })
}

fn solve_line(line: &str) -> i64 {
    let parts: Vec<&str> = line.split(" | ").collect();

    let key = make_key(parts[0]);

    decode(parts[1], key)
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(
        file.par_lines()
            .fold(|| 0, |acc, line| acc + solve_line(line))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_8a_solution() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day8.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();
        assert_eq!(Solution::Integer(349), part_a(&file))
    }

    #[test]
    fn day_8b_solution() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day8.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();
        assert_eq!(Solution::Integer(1070957), part_b(&file))
    }

    #[test]
    fn part_b_example() {
        assert_eq!(Solution::Integer(5353), part_b("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));
    }
}

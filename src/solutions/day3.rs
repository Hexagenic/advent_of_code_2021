use crate::solutions::Solution;

#[derive(Debug)]
struct BitCounter {
    bits: Vec<u64>,
    lines: u64,
}

impl BitCounter {
    fn new() -> Self {
        BitCounter {
            bits: vec![],
            lines: 0,
        }
    }
}

fn common_bits(counter: &BitCounter) -> (i64, i64) {
    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;
    let threshold = counter.lines / 2;

    for (i, b) in counter.bits.iter().enumerate() {
        if *b >= threshold {
            gamma += 1 << (counter.bits.len() - i - 1);
        }
        if *b <= threshold {
            epsilon += 1 << (counter.bits.len() - i - 1);
        }
    }

    (gamma, epsilon)
}

fn fold_func(acc: BitCounter, value: &str) -> BitCounter {
    let mut acc = acc;
    if acc.bits.len() < value.len() {
        acc.bits.resize(value.len(), 0);
    }

    for (i, c) in value.chars().enumerate() {
        if c == '1' {
            acc.bits[i] += 1;
        }
    }

    acc.lines += 1;

    acc
}

pub fn part_a(file: &str) -> Solution {
    let counter = file
        .split_ascii_whitespace()
        .fold(BitCounter::new(), fold_func);

    let (gamma, epsilon) = common_bits(&counter);
    Solution::Integer(gamma * epsilon)
}

fn life_support(data: &[&str], oxygen_generator: bool) -> i64 {
    use std::cmp::Ordering;

    let bits = data[0].len();
    let mut filtered: Vec<&str> = data.to_vec();
    let mut filter_string: String = String::from("");
    for i in 0..bits {
        let counter = filtered
            .iter()
            .fold(BitCounter::new(), |acc, l| fold_func(acc, l));

        let one_bits = counter.bits[i];
        let zero_bits = counter.lines - one_bits;

        filter_string.push(match one_bits.cmp(&zero_bits) {
            Ordering::Greater | Ordering::Equal => {
                if oxygen_generator {
                    '1'
                } else {
                    '0'
                }
            }
            Ordering::Less => {
                if oxygen_generator {
                    '0'
                } else {
                    '1'
                }
            }
        });

        filtered = filtered
            .iter()
            .filter(|s| s.starts_with(&filter_string))
            .copied()
            .collect();

        if filtered.len() == 1 {
            filter_string = filtered[0].to_string();
            break;
        }
    }

    i64::from_str_radix(filter_string.as_str(), 2).unwrap()
}

pub fn part_b(file: &str) -> Solution {
    let data: Vec<&str> = file.split_ascii_whitespace().collect();
    Solution::Integer((life_support(&data, true) * life_support(&data, false)) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_counter() {
        let file = "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010";
        let counter = file.split_whitespace().fold(BitCounter::new(), fold_func);

        assert_eq!(7, counter.bits[0]);
        assert_eq!(5, counter.bits[1]);
        assert_eq!(8, counter.bits[2]);
        assert_eq!(7, counter.bits[3]);

        let (gamma, epsilon) = common_bits(&counter);

        assert_eq!(0b10110, gamma);
        assert_eq!(0b01001, epsilon);
    }

    #[test]
    fn part_b_example() {
        let file = "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010";
        assert_eq!(Solution::Integer(230), part_b(file));
    }

    #[test]
    fn oxygen_generator() {
        let file: Vec<&str> =
            "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010"
                .split_whitespace()
                .collect();
        assert_eq!(23, life_support(&file, true));
    }

    #[test]
    fn co2_scrubber() {
        let file: Vec<&str> =
            "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010"
                .split_whitespace()
                .collect();
        assert_eq!(10, life_support(&file, false));
    }
}

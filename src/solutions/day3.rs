struct BitCounter {
    bits: Vec<u16>,
    lines: u16,
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
        } else {
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

pub fn part_a(file: &str) -> i64 {
    let counter = file
        .split_ascii_whitespace()
        .fold(BitCounter::new(), fold_func);

    let (gamma, epsilon) = common_bits(&counter);
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010";
        let counter = file
            .split_ascii_whitespace()
            .fold(BitCounter::new(), fold_func);

        assert_eq!(7, counter.bits[0]);
        assert_eq!(5, counter.bits[1]);
        assert_eq!(8, counter.bits[2]);
        assert_eq!(7, counter.bits[3]);

        let (gamma, epsilon) = common_bits(&counter);

        assert_eq!(0b10110, gamma);
        assert_eq!(0b01001, epsilon);
    }
}

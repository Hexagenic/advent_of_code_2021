struct Board {
    numbers: Vec<u8>,
}

impl Board {
    fn parse(lines: &[&str]) -> Board {
        Board {
            numbers: lines
                .iter()
                .copied()
                .flat_map(str::split_whitespace)
                .filter(|s| !s.is_empty())
                .flat_map(str::parse::<u8>)
                .collect(),
        }
    }

    fn bingo_index(&self, numbers: &[u8]) -> (usize, i64) {
        let mut bingo_marks: u64 = 0;
        let mut numbers_left = self.numbers.clone();

        for (index, number) in numbers.iter().enumerate() {
            if let Some(n) = self.numbers.iter().position(|n| n == number) {
                bingo_marks |= 1 << n;
                numbers_left.retain(|value| *value != *number);

                if Self::check_bingo(bingo_marks) {
                    let sum_unmarked = numbers_left.iter().map(|x| i64::from(*x)).sum::<i64>();
                    let score = i64::from(*number) * sum_unmarked;
                    return (index, score);
                }
            }
        }

        (usize::MAX, 0)
    }

    #[inline]
    fn check_bingo(marks: u64) -> bool {
        Self::check_bingo_line(marks, 0b0_0000_0000_0000_0000_0001_1111)
            || Self::check_bingo_line(marks, 0b0_0000_0000_0000_0011_1110_0000)
            || Self::check_bingo_line(marks, 0b0_0000_0000_0111_1100_0000_0000)
            || Self::check_bingo_line(marks, 0b0_0000_1111_1000_0000_0000_0000)
            || Self::check_bingo_line(marks, 0b1_1111_0000_0000_0000_0000_0000)
            || Self::check_bingo_line(marks, 0b0_0001_0000_1000_0100_0010_0001)
            || Self::check_bingo_line(marks, 0b0_0010_0001_0000_1000_0100_0010)
            || Self::check_bingo_line(marks, 0b0_0100_0010_0001_0000_1000_0100)
            || Self::check_bingo_line(marks, 0b0_1000_0100_0010_0001_0000_1000)
            || Self::check_bingo_line(marks, 0b1_0000_1000_0100_0010_0001_0000)
    }

    #[inline]
    fn check_bingo_line(marks: u64, line: u64) -> bool {
        (marks & line) == line
    }
}

pub fn part_a(file: &str) -> i64 {
    let lines: Vec<&str> = file.lines().collect();

    let first_line: Vec<u8> = lines[0].split(',').flat_map(str::parse).collect();

    (&lines[2..])
        .chunks(6)
        .map(Board::parse)
        .map(|b| b.bingo_index(&first_line))
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1 as i64
}

pub fn part_b(file: &str) -> i64 {
    let lines: Vec<&str> = file.lines().collect();

    let first_line: Vec<u8> = lines[0].split(',').flat_map(str::parse).collect();

    (&lines[2..])
        .chunks(6)
        .map(Board::parse)
        .map(|b| b.bingo_index(&first_line))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day4_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(4512, part_a(&file));
    }
}

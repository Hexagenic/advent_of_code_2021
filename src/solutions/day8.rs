// Segments by number
// 0 => 6
// 1 => 2
// 2 => 5
// 3 => 5
// 4 => 4
// 5 => 5
// 6 => 6
// 7 => 3
// 8 => 7
// 9 => 6

// Numbers by segment
//
// 2 => 1
// 3 => 7
// 4 => 4
// 5 => 2, 3, 5
// 6 => 0, 6, 9
// 7 => 8

// 3 shares two segments with 1
// 7 shares two segments with 1
// Shares

pub fn part_a(file: &str) -> i64 {
    fn count_easy(acc: i64, output: &str) -> i64 {
        acc + (output
            .split_ascii_whitespace()
            .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
            .count() as i64)
    }

    file.lines()
        .filter_map(|l| l.split(" | ").nth(1))
        .fold(0, count_easy)
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
        assert_eq!(349, part_a(&file))
    }
}

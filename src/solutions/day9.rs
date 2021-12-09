pub fn part_a(file: &str) -> i64 {
    let data: Vec<Vec<u8>> = file
        .lines()
        .map(|l| l.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let xs = data.len();
    let ys = data[0].len();

    let mut sum: i64 = 0;

    for x in 0..xs {
        for y in 0..ys {
            let smaller_than_left = x == 0 || data[x - 1][y] > data[x][y];
            let smaller_than_top = y == 0 || data[x][y - 1] > data[x][y];
            let smaller_than_right = x == (xs - 1) || data[x + 1][y] > data[x][y];
            let smaller_than_bottom = y == (ys - 1) || data[x][y + 1] > data[x][y];

            if smaller_than_left && smaller_than_top && smaller_than_right && smaller_than_bottom {
                sum += i64::from(data[x][y]) + 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day9_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(15, part_a(&file));
    }
}

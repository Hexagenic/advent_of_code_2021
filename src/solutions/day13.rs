pub fn part_a(file: &str) -> i64 {
    let mut dots: Vec<(i32, i32)> = vec![];

    let mut isParsingNumbers = true;

    for foo in file.lines() {
        if foo.is_empty() {
            isParsingNumbers = false;
            continue;
        }

        if isParsingNumbers {
            dots.push((1, 1));
        }
    }

    dots.len() as i64
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

        assert_eq!(17, part_a(&file));
    }
}

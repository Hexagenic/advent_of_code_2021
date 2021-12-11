fn matching_tag(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '>' => '<',
        '}' => '{',
        _ => panic!("Unknown character"),
    }
}

fn first_error(line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        match c {
            '(' | '<' | '[' | '{' => stack.push(c),
            ')' | '>' | ']' | '}' => {
                if stack.last() == Some(&matching_tag(c)) {
                    stack.pop();
                } else {
                    return Some(c);
                }
            }
            _ => (),
        }
    }

    None
}

fn stack_repair(line: &str) -> i64 {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        match c {
            '(' | '<' | '[' | '{' => stack.push(c),
            ')' | '>' | ']' | '}' => {
                if stack.last() == Some(&matching_tag(c)) {
                    stack.pop();
                } else {
                    return 0;
                }
            }
            _ => (),
        }
    }

    stack
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + score_of_fix(*c))
}

fn score_of_error(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_of_fix(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => -1,
    }
}

pub fn part_a(file: &str) -> i64 {
    file.lines()
        .filter_map(first_error)
        .map(score_of_error)
        .sum()
}

pub fn part_b(file: &str) -> i64 {
    let mut fixes: Vec<i64> = file.lines().map(stack_repair).filter(|v| v > &0).collect();
    fixes.sort_unstable();

    fixes[fixes.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_example1() {
        assert_eq!(Some('}'), first_error("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(Some(')'), first_error("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(Some(']'), first_error("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(Some(')'), first_error("[<(<(<(<{}))><([]([]()"));
        assert_eq!(Some('>'), first_error("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn day9_part1_example2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day10_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(26397, part_a(&file));
    }

    #[test]
    fn day9_part2_example1() {
        assert_eq!(288957, stack_repair("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(5566, stack_repair("[(()[<>])]({[<{<<[]>>("));
        assert_eq!(1480781, stack_repair("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!(995444, stack_repair("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!(294, stack_repair("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn day9_part2_example2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day10_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(288957, part_b(&file));
    }
}

fn first_error(line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        match c {
            '(' | '<' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                } else {
                    return Some(')');
                }
            }
            '>' => {
                if stack.last() == Some(&'<') {
                    stack.pop();
                } else {
                    return Some('>');
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    return Some(']');
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return Some('}');
                }
            }
            _ => (),
        }
    }

    None
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

pub fn part_a(file: &str) -> i64 {
    file.lines().filter_map(first_error).map(score_of_error).sum()
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
}

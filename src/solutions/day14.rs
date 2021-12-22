use crate::solutions::Solution;
use std::collections::HashMap;

type Rule = ((char, char), char);

#[derive(Debug, Clone)]
struct State {
    pairs: HashMap<(char, char), i64>,
    chars: HashMap<char, i64>,
}

fn increment(map: &mut HashMap<char, i64>, key: char, val: i64) {
    if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key) {
        e.insert(val);
    } else {
        *map.get_mut(&key).unwrap() += val;
    }
}

fn increment_pair(map: &mut HashMap<(char, char), i64>, key: (char, char), val: i64) {
    if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key) {
        e.insert(val);
    } else {
        *map.get_mut(&key).unwrap() += val;
    }
}

fn parse_rule(line: &str) -> Rule {
    let parts = line.split(" -> ").collect::<Vec<_>>();
    let mut pair = parts[0].chars();
    let new_char = parts[1].chars().next().unwrap();

    ((pair.next().unwrap(), pair.next().unwrap()), new_char)
}

fn state_from_template(template: &str) -> State {
    let mut pairs = HashMap::new();
    let mut chars = HashMap::new();

    for i in 0..template.len() {
        let c1 = template.chars().nth(i).unwrap();

        increment(&mut chars, c1, 1);

        if i == 0 {
            continue;
        }

        let c0 = template.chars().nth(i - 1).unwrap();

        increment_pair(&mut pairs, (c0, c1), 1);
    }

    State { pairs, chars }
}

fn iterate(state: &State, rules: &[Rule]) -> State {
    let mut new_state = state.clone();

    for rule in rules {
        if let Some(count) = state.pairs.get(&rule.0) {
            increment(&mut new_state.chars, rule.1, *count);
            increment_pair(&mut new_state.pairs, (rule.0 .0, rule.1), *count);
            increment_pair(&mut new_state.pairs, (rule.1, rule.0 .1), *count);
            increment_pair(&mut new_state.pairs, (rule.0 .0, rule.0 .1), -*count);
        }
    }

    new_state
}

fn n_iterations(initial_state: State, rules: &[Rule], iterations: i64) -> State {
    let mut state = initial_state;
    for _ in 0..iterations {
        state = iterate(&state, rules);
    }

    state
}

pub fn part_a(file: &str) -> Solution {
    let mut lines = file.lines();
    let template = &lines.next().unwrap();
    let state = state_from_template(template);

    let rules = lines.skip(1).map(parse_rule).collect::<Vec<_>>();
    let state = n_iterations(state, &rules, 10);

    let count = state.chars.iter().fold((i64::MAX, i64::MIN), |acc, v| {
        (i64::min(acc.0, *v.1), i64::max(acc.1, *v.1))
    });
    Solution::Integer(count.1 - count.0)
}

pub fn part_b(file: &str) -> Solution {
    let mut lines = file.lines();
    let template = &lines.next().unwrap();
    let state = state_from_template(template);

    let rules = lines.skip(1).map(parse_rule).collect::<Vec<_>>();
    let state = n_iterations(state, &rules, 40);

    let count = state.chars.iter().fold((i64::MAX, i64::MIN), |acc, v| {
        (i64::min(acc.0, *v.1), i64::max(acc.1, *v.1))
    });
    Solution::Integer(count.1 - count.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day14_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(Solution::Integer(1588), part_a(&file));
    }

    #[test]
    fn example_2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day14_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(Solution::Integer(2188189693529), part_b(&file));
    }
}

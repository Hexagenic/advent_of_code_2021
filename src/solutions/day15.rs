use crate::solutions::Solution;
use std::collections::HashSet;

type NodeSet = HashSet<usize>;

fn get_lowest_f_score(open_set: &NodeSet, f_score: &[usize]) -> usize {
    *open_set
        .iter()
        .min_by(|a, b| f_score[**a].cmp(&f_score[**b]))
        .unwrap()
}

fn distance(a: &(i64, i64), b: &(i64, i64)) -> usize {
    (i64::abs(a.0 - b.0) + i64::abs(a.1 - b.1)) as usize
}

fn get_neighbours(pos: &(i64, i64), width: i64, height: i64) -> Vec<usize> {
    let mut neighbours = vec![];
    let ds = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for d in ds {
        let x = pos.0 + d.0;
        let y = pos.1 + d.1;

        if x < 0 || y < 0 || x >= width || y >= height {
            continue;
        }
        let i = (x + y * width) as usize;
        neighbours.push(i);
    }

    neighbours
}

pub fn a_star(map: &[u8], start_pos: (i64, i64), end_pos: (i64, i64)) -> usize {
    let start: usize = 0;

    let width = end_pos.0 + 1;
    let height = end_pos.1 + 1;

    let goal: usize = (end_pos.0 + width * end_pos.1) as usize;

    let mut open_set: NodeSet = NodeSet::new();
    open_set.insert(start);

    let mut g_score: Vec<usize> = vec![usize::MAX; map.len()];
    g_score[start] = 0;

    let mut f_score: Vec<usize> = vec![usize::MAX; map.len()];
    f_score[start] = distance(&start_pos, &end_pos);

    while !open_set.is_empty() {
        let current = get_lowest_f_score(&open_set, &f_score);

        if current == goal {
            return g_score[current];
        }

        open_set.remove(&current);

        let current_pos = ((current as i64) % width, (current as i64) / width);

        for neighbour in get_neighbours(&current_pos, width, height) {
            let tentative_g_score = g_score[current] + (map[neighbour] as usize);
            if tentative_g_score < g_score[neighbour] {
                g_score[neighbour] = tentative_g_score;
                f_score[neighbour] = g_score[neighbour] + distance(&current_pos, &end_pos);

                if !open_set.iter().any(|n| n == &neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }

    usize::MAX
}

fn expand_map(map: &[u8]) -> Vec<u8> {
    let width = (map.len() as f64).sqrt() as usize;

    let mut out_map = vec![0; width * width * 25];

    for x in 0..width {
        for y in 0..width {
            let i = x + width * y;
            for mx in 0..5 {
                for my in 0..5 {
                    let fx = mx * width + x;
                    let fy = my * width + y;

                    let fi = fx + (width * 5) * fy;
                    out_map[fi] = ((((map[i] as usize) + mx + my - 1) % 9) + 1) as u8;
                }
            }
        }
    }

    out_map
}

pub fn part_a(file: &str) -> Solution {
    let map = file
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect::<Vec<_>>();
    let width = (map.len() as f64).sqrt() as i64;
    Solution::Integer(a_star(&map, (0, 0), (width - 1, width - 1)) as i64)
}

pub fn part_b(file: &str) -> Solution {
    let map = file
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect::<Vec<_>>();
    let map = expand_map(&map);
    let width = (map.len() as f64).sqrt() as i64;
    Solution::Integer(a_star(&map, (0, 0), (width - 1, width - 1)) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day15_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(Solution::Integer(40), part_a(&file));
    }
    #[test]
    fn example_2() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day15_test.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();

        assert_eq!(Solution::Integer(315), part_b(&file));
    }
}

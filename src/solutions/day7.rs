pub fn part_a(file: &str) -> i64 {
    let crabs: Vec<i32> = file.split(',').flat_map(str::parse).collect();

    let mut shortest: i32 = i32::MAX;

    for l in &crabs {
        let current = crabs.iter().map(|c| i32::abs(c - l)).sum();
        if current < shortest {
            shortest = current;
        }
    }

    i64::from(shortest)
}

fn sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn part_b(file: &str) -> i64 {
    let crabs: Vec<i32> = file.split(',').flat_map(str::parse).collect();

    let mut shortest: i32 = i32::MAX;

    for l in &crabs {
        let current = crabs.iter().map(|c| sum(i32::abs(c - l - 1))).sum();
        if current < shortest {
            shortest = current;
        }
    }

    i64::from(shortest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_a() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day7.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();
        assert_eq!(336120, part_a(&file));
    }

    #[test]
    fn solution_b() {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day7.txt");
        let file: String = std::fs::read_to_string(&path).unwrap();
        assert_eq!(96864235, part_b(&file));
    }

    #[test]
    fn example_a() {
        assert_eq!(37, part_a("16,1,2,0,4,2,7,1,2,14"));
    }

    #[test]
    fn example_b() {
        assert_eq!(168, part_b("16,1,2,0,4,2,7,1,2,14"));
    }

    #[test]
    fn zero_distance() {
        assert_eq!(0, part_a("16"));
        assert_eq!(0, part_a("16,16"))
    }
    #[test]
    fn distance_one() {
        assert_eq!(2, part_a("16,14"))
    }
}

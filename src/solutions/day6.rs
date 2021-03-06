use crate::solutions::Solution;

type FishPopulation = [i64; 9];

fn parse_population(input: &str) -> FishPopulation {
    input
        .split(',')
        .flat_map(str::parse::<usize>)
        .fold([0; 9], |mut acc, x| {
            acc[x] += 1;
            acc
        })
}

fn simulate_step(population: FishPopulation) -> FishPopulation {
    [
        population[1],
        population[2],
        population[3],
        population[4],
        population[5],
        population[6],
        population[7] + population[0],
        population[8],
        population[0],
    ]
}

fn simulate_n_steps(first_generation: FishPopulation, steps: u32) -> FishPopulation {
    let mut population = first_generation;
    for _ in 0..steps {
        population = simulate_step(population);
    }

    population
}

fn count_fish(population: FishPopulation) -> i64 {
    population.iter().sum()
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(count_fish(simulate_n_steps(parse_population(file), 80)))
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(count_fish(simulate_n_steps(parse_population(file), 256)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_population_test() {
        assert_eq!([0, 1, 1, 2, 1, 0, 0, 0, 0], parse_population("3,4,3,1,2"));
    }

    #[test]
    fn part_1_example() {
        assert_eq!(
            26,
            count_fish(simulate_n_steps(parse_population("3,4,3,1,2"), 18))
        );
        assert_eq!(
            5934,
            count_fish(simulate_n_steps(parse_population("3,4,3,1,2"), 80))
        );
    }
}

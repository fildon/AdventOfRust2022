use std::collections::HashSet;

use crate::utils::get_lines;

fn split_in_half<T: std::clone::Clone + std::hash::Hash + std::cmp::Eq>(
    rucksack: Vec<T>,
) -> Vec<HashSet<T>> {
    let compartment_size = rucksack.len() / 2;
    let first_half = HashSet::from_iter(rucksack[..compartment_size].iter().cloned());
    let second_half = HashSet::from_iter(rucksack[compartment_size..].iter().cloned());
    vec![first_half, second_half]
}

fn find_duplicate(containers: Vec<HashSet<char>>) -> char {
    containers
        .iter()
        .skip(1)
        .fold(containers[0].clone(), |a, b| {
            a.intersection(b).cloned().collect()
        })
        .into_iter()
        .next()
        .unwrap()
}

pub fn solve_part1(file_path: &str) -> u32 {
    get_lines(file_path)
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(split_in_half)
        .map(find_duplicate)
        .map(|char| {
            if char.is_lowercase() {
                (char as u32) - 97 + 1
            } else {
                (char as u32) - 65 + 27
            }
        })
        .sum()
}

pub fn solve_part2(file_path: &str) -> u32 {
    get_lines(file_path)
        .iter()
        .map(|line| HashSet::from_iter(line.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .map(find_duplicate)
        .map(|char| {
            if char.is_lowercase() {
                (char as u32) - 97 + 1
            } else {
                (char as u32) - 65 + 27
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            solve_part1("src/day03rucksackreorganization/input-test.txt"),
            157
        );
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(
            solve_part1("src/day03rucksackreorganization/input-real.txt"),
            8018
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            solve_part2("src/day03rucksackreorganization/input-test.txt"),
            70
        );
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(
            solve_part2("src/day03rucksackreorganization/input-real.txt"),
            2518
        );
    }
}

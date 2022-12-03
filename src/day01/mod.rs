use std::fs;

pub fn solve_part1(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .unwrap()
        .split("\r\n\r\n")
        .into_iter()
        .map(|inventory_lines| {
            inventory_lines
                .lines()
                .into_iter()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part1("src/day01/input-test.txt"), 24000);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part1("src/day01/input-real.txt"), 70698);
    }
}

use crate::utils::get_file_contents;

pub fn solve_part1(file_path: &str) -> u32 {
    get_file_contents(file_path)
        .split("\n\n")
        .map(|inventory_lines| {
            inventory_lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn solve_part2(file_path: &str) -> u32 {
    let mut inventories = get_file_contents(file_path)
        .split("\n\n")
        .into_iter()
        .map(|inventory_lines| {
            inventory_lines
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    inventories.sort_unstable_by(|a, b| b.cmp(a));
    return inventories.iter().take(3).sum();
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

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part2("src/day01/input-test.txt"), 45000);
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part2("src/day01/input-real.txt"), 206643);
    }
}

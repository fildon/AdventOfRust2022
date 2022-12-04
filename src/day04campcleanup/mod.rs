use crate::utils::get_lines;

fn to_range(assignment: &str) -> (u8, u8) {
    let (lower, upper) = assignment.split_once("-").unwrap();
    (lower.parse().unwrap(), upper.parse().unwrap())
}

fn to_range_pair(line: &str) -> ((u8, u8), (u8, u8)) {
    let (a, b) = line.split_once(",").unwrap();
    (to_range(a), to_range(b))
}

pub fn solve_part1(file_path: &str) -> usize {
    get_lines(file_path)
        .iter()
        .map(|line| to_range_pair(line))
        .filter(|((a_start, a_end), (b_start, b_end))| {
            (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end)
        })
        .count()
}

pub fn solve_part2(file_path: &str) -> usize {
    get_lines(file_path)
        .iter()
        .map(|line| to_range_pair(line))
        .filter(|((a_start, a_end), (b_start, b_end))| {
            (a_start <= b_end && a_end >= b_start) || (b_start <= a_end && b_end >= a_start)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part1("src/day04campcleanup/input-test.txt"), 2);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part1("src/day04campcleanup/input-real.txt"), 503);
    }
    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part2("src/day04campcleanup/input-test.txt"), 4);
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part2("src/day04campcleanup/input-real.txt"), 827);
    }
}

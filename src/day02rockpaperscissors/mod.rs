use crate::utils::get_lines;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn shape_score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

/**
 * 0 if you lost, 3 if the round was a draw, and 6 if you won
 */
fn outcome_score(round: (&Shape, &Shape)) -> u32 {
    match round {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => 6,
    }
}

pub fn solve_part1(file_path: &str) -> u32 {
    get_lines(file_path)
        .iter()
        .map(|line| match line.as_str() {
            "A X" => (Shape::Rock, Shape::Rock),
            "A Y" => (Shape::Rock, Shape::Paper),
            "A Z" => (Shape::Rock, Shape::Scissors),
            "B X" => (Shape::Paper, Shape::Rock),
            "B Y" => (Shape::Paper, Shape::Paper),
            "B Z" => (Shape::Paper, Shape::Scissors),
            "C X" => (Shape::Scissors, Shape::Rock),
            "C Y" => (Shape::Scissors, Shape::Paper),
            "C Z" => (Shape::Scissors, Shape::Scissors),
            _ => panic!("Unrecognised symbol"),
        })
        .map(|(theirs, mine)| shape_score(&mine) + outcome_score((&theirs, &mine)))
        .sum()
}

pub fn solve_part2(file_path: &str) -> u32 {
    get_lines(file_path)
        .iter()
        .map(|line| match line.as_str() {
            "A X" => (Shape::Rock, Shape::Scissors),
            "A Y" => (Shape::Rock, Shape::Rock),
            "A Z" => (Shape::Rock, Shape::Paper),
            "B X" => (Shape::Paper, Shape::Rock),
            "B Y" => (Shape::Paper, Shape::Paper),
            "B Z" => (Shape::Paper, Shape::Scissors),
            "C X" => (Shape::Scissors, Shape::Paper),
            "C Y" => (Shape::Scissors, Shape::Scissors),
            "C Z" => (Shape::Scissors, Shape::Rock),
            _ => panic!("Unrecognised symbol"),
        })
        .map(|(theirs, mine)| shape_score(&mine) + outcome_score((&theirs, &mine)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part1("src/day02rockpaperscissors/input-test.txt"), 15);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(
            solve_part1("src/day02rockpaperscissors/input-real.txt"),
            14069
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part2("src/day02rockpaperscissors/input-test.txt"), 12);
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(
            solve_part2("src/day02rockpaperscissors/input-real.txt"),
            12411
        );
    }
}

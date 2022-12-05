use crate::utils::get_file_contents;

struct Instruction {
    qty: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;

struct Job {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

fn parse_file(file_path: &str) -> Job {
    let file_contents = get_file_contents(file_path);
    let (stacks_section, instruction_lines) = file_contents.split_once("\n\n").unwrap();

    let stacks_lines = stacks_section.split("\n").collect::<Vec<_>>();
    let stack_count = 1 + (stacks_lines[0].len() - 3) / 4;

    let mut stacks = Vec::<Vec<char>>::new();
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    // Little hack to iterate through all but last item... (which is a garbage row)
    let mut stacks_iterator = stacks_lines.iter().peekable();
    while let Some(line) = stacks_iterator.next() {
        if stacks_iterator.peek().is_some() {
            let chars = line.split("").collect::<Vec<_>>();
            let mut stack_pointer = 0;
            for input_pointer in (1..chars.len()).step_by(4) {
                let crate_label = chars[input_pointer + 1];
                if crate_label != " " {
                    stacks[stack_pointer].push(chars[input_pointer + 1].chars().nth(0).unwrap());
                }
                stack_pointer += 1;
            }
        }
    }

    let instructions = instruction_lines
        .split("\n")
        .map(|line| {
            line.split(" ")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|instruction_parts| Instruction {
            qty: instruction_parts[0],
            from: instruction_parts[1] - 1,
            to: instruction_parts[2] - 1,
        })
        .collect::<Vec<_>>();

    Job {
        stacks,
        instructions,
    }
}

fn execute_instruction(mut stacks: Vec<Stack>, instruction: &Instruction) -> Vec<Stack> {
    let mut moving_slice = stacks[instruction.from][..instruction.qty].to_vec();
    moving_slice.reverse();

    stacks[instruction.from] = stacks[instruction.from][instruction.qty..].to_vec();
    stacks[instruction.to].splice(0..0, moving_slice);

    stacks
}

pub fn solve_part1(file_path: &str) -> String {
    let job = parse_file(file_path);

    job.instructions
        .iter()
        .fold(job.stacks, execute_instruction)
        .iter()
        .map(|stack| stack[0])
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part1("src/day05supplystacks/input-test.txt"), "CMZ");
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(
            solve_part1("src/day05supplystacks/input-real.txt"),
            "SHMSDGZVC"
        );
    }
    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(solve_part2("src/day05supplystacks/input-test.txt"), "MCD");
    // }
    // #[test]
    // fn part2_real_input() {
    //     assert_eq!(
    //         solve_part2("src/day05supplystacks/input-real.txt"),
    //         "VRZGHDFBQ"
    //     );
    // }
}

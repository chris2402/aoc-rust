use anyhow::anyhow;
use std::{ops::Add, str::FromStr};

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction character: {}", value)),
        }
    }
}

struct DialInstruction {
    direction: Direction,
    steps: usize,
}

impl DialInstruction {
    fn move_from(&self, value: isize) -> (isize, isize) {
        let difference = match self.direction {
            Direction::Right => value + self.steps as isize,
            Direction::Left => value - self.steps as isize,
        };

        let n_rollovers = difference.div_euclid(100).abs();

        let moved_value = if difference < 0 {
            100 + (difference % 100)
        } else {
            difference
        } % 100;

        (moved_value, n_rollovers)
    }
}

impl FromStr for DialInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.chars();

        let direction = s_iter
            .nth(0)
            .and_then(|c| Direction::try_from(c).ok())
            .unwrap();

        let steps: usize = s_iter.collect::<String>().parse()?;

        Ok(DialInstruction { direction, steps })
    }
}

impl Add<isize> for &DialInstruction {
    type Output = isize;

    fn add(self, rhs: isize) -> Self::Output {
        let (r, _) = self.move_from(rhs);
        r
    }
}

struct DialInstructions(Vec<DialInstruction>);

impl DialInstructions {
    fn iter(&self) -> std::slice::Iter<'_, DialInstruction> {
        self.0.iter()
    }

    fn iter_values_from(&self, start: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
        let mut current = start;
        self.iter().map(move |instr| {
            let movement @ (new_current, _) = instr.move_from(current);
            current = new_current;

            movement
        })
    }
}

impl TryFrom<&str> for DialInstructions {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let instructions = s
            .lines()
            .map(|line| line.trim().parse::<DialInstruction>())
            .collect::<Result<Vec<DialInstruction>, anyhow::Error>>()?;

        Ok(DialInstructions(instructions))
    }
}

fn main() -> anyhow::Result<()> {
    println!("Dec 1, 2025 - Dial Instructions");
    let result = DialInstructions::try_from(
        std::fs::read_to_string("packages/dec_1/input.txt")
            .expect("Couldn't read input.txt")
            .as_str(),
    )?
    .iter_values_from(50)
    .map(|(_, v)| v)
    .sum::<isize>();

    println!("Final position: {}", result);
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_left_instructionp() {
        let line = "L2";
        let instruction: DialInstruction = line.parse().unwrap();
        assert!(matches!(instruction.direction, Direction::Left));
        assert_eq!(instruction.steps, 2);
    }

    #[test]
    fn it_parses_a_right_instruction() {
        let line = "R100";
        let instruction: DialInstruction = line.parse().unwrap();
        assert!(matches!(instruction.direction, Direction::Right));
        assert_eq!(instruction.steps, 100);
    }

    #[test]
    fn it_parses_a_list_of_instructions() {
        const INTPUT: &str = "L3\nR2\nL5\nR10";
        let instructions: DialInstructions = INTPUT.try_into().unwrap();

        let mut iter = instructions.iter();
        let first = iter.next().unwrap();
        assert!(matches!(first.direction, Direction::Left));
        assert_eq!(first.steps, 3);
        let second = iter.next().unwrap();
        assert!(matches!(second.direction, Direction::Right));
        assert_eq!(second.steps, 2);
        let third = iter.next().unwrap();
        assert!(matches!(third.direction, Direction::Left));
        assert_eq!(third.steps, 5);
        let fourth = iter.next().unwrap();
        assert!(matches!(fourth.direction, Direction::Right));
        assert_eq!(fourth.steps, 10);
        assert!(iter.next().is_none());
    }

    #[test]
    fn it_subtracts_on_a_left_instruction() {
        let instruction = DialInstruction {
            direction: Direction::Left,
            steps: 10,
        };

        let result = &instruction + 14;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_subtracts_on_a_left_instruction_rollsover_() {
        let instruction = DialInstruction {
            direction: Direction::Left,
            steps: 1,
        };

        let result = &instruction + 0;
        assert_eq!(result, 99);
    }

    #[test]
    fn it_adds_on_a_right_instruction() {
        let instruction = DialInstruction {
            direction: Direction::Right,
            steps: 10,
        };

        let result = &instruction + 4;
        assert_eq!(result, 14);
    }

    #[test]
    fn it_adds_on_a_right_instruction_rollsover() {
        let instruction = DialInstruction {
            direction: Direction::Right,
            steps: 1,
        };

        let result = &instruction + 99;
        assert_eq!(result, 0);
    }
    #[test]
    fn it_passes_test_input_1() {
        const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let instructions: DialInstructions = INPUT.try_into().unwrap();

        let result = instructions
            .iter_values_from(50)
            .filter(|&v| v.0 == 0)
            .count();
        assert_eq!(result, 3);
    }

    #[test]
    fn it_passes_test_input_2() {
        const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let instructions: DialInstructions = INPUT.try_into().unwrap();

        let result: isize = instructions.iter_values_from(50).map(|(_, v)| v).sum();

        assert_eq!(result, 6);
    }
}

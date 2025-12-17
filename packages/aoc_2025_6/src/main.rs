use std::str::FromStr;

use array2d::Array2D;

fn main() {
    let input = std::fs::read_to_string("packages/aoc_2025_6/input.txt")
        .expect("Failed to read input file");
    let result_1 = solve_part_1(&input).expect("Failed to solve part 1");
    println!("Part 1: {}", result_1);
}

fn solve_part_1(_input: &str) -> Result<usize, anyhow::Error> {
    let math_tasks = _input.parse::<MathTasks>()?;
    Ok(math_tasks.solve_all())
}

struct MathTasks(Vec<MathTask>);

impl MathTasks {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<'_, MathTask> {
        self.0.iter()
    }

    fn solve_all(&self) -> usize {
        self.0.iter().map(|task| task.solve()).sum()
    }
}

impl FromStr for MathTasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let numbers = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<Vec<usize>>, _>>()?;

        let array_2d: Array2D<usize> = Array2D::from_rows(&numbers)?;
        let operators = lines
            .iter()
            .last()
            .ok_or(anyhow::anyhow!("Failed to get last line of operators!"))?
            .split_whitespace()
            .map(|op| op.parse::<Operator>())
            .collect::<Result<Vec<Operator>, _>>()?;

        Ok(MathTasks(
            operators
                .iter()
                .enumerate()
                .map(|(i, op)| -> Result<MathTask, anyhow::Error> {
                    let numbers = array_2d.column_iter(i)?;
                    Ok(MathTask {
                        numbers: numbers.cloned().collect(),
                        operator: *op,
                    })
                })
                .collect::<Result<Vec<MathTask>, _>>()?,
        ))
    }
}

struct MathTask {
    numbers: Vec<usize>,
    operator: Operator,
}

impl MathTask {
    fn solve(&self) -> usize {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(anyhow::anyhow!("Unknown operator: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn it_parses() {
        let math_tasks = INPUT.parse::<MathTasks>().expect("Failed to parse input");

        assert_eq!(math_tasks.len(), 4);
        assert_eq!(
            math_tasks
                .iter()
                .filter(|mt| matches!(mt.operator, Operator::Add))
                .count(),
            2
        );
        assert_eq!(
            math_tasks
                .iter()
                .filter(|mt| matches!(mt.operator, Operator::Multiply))
                .count(),
            2
        );
    }

    #[test]
    fn it_solves_1() {
        let math_tasks = INPUT.parse::<MathTasks>().expect("Failed to parse input");

        let individual_results: Vec<usize> = math_tasks.iter().map(|mt| mt.solve()).collect();
        let result: usize = math_tasks.solve_all();
        assert_eq!(individual_results, vec![33210, 490, 4243455, 401]);
        assert_eq!(result, 4277556);
    }
}

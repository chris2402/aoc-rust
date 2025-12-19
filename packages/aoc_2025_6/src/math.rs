mod cephalopod;
mod human;

use std::str::FromStr;

use crate::number::CharGrid;
use anyhow::anyhow;

pub struct MathTasks {
    char_grid: CharGrid,
    operators: Vec<Operator>,
}

impl MathTasks {
    fn len(&self) -> usize {
        self.operators.len()
    }

    fn try_solve_normal(&self) -> Result<usize, anyhow::Error> {
        let number_array =
            HumanMathParser::read(self.char_grid.horizontal_iter(), self.operators.clone())?;

        // TODO
        Ok(number_array.iter().map(|task| task.solve()).sum())
    }
}

/*
 * Right before giving up I had an idea:
 *  Parsers that generate vectors of iterators that over the CharGrid
 *   - HumanParser parses by folding over each number into its own vector of usize;
 *      each vector is one math equation, zipped with the operator
 *   - CephalopodParser pushes all line chars into their own String, even when it is whitespace;
 *      split iterator on empty strings, each split is one math equation that can be zipped with the operator
 */
struct HumanMathParser;

impl HumanMathParser {
    fn read<T: Iterator<Item = String>>(
        lines: T,
        operators: Vec<Operator>,
    ) -> Result<Vec<MathTask>, anyhow::Error> {
        let math_numbers = lines
            .map(|l| {
                l.split_whitespace()
                    .map(usize::from_str)
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<Vec<usize>>, _>>()?
            .iter()
            .try_fold(vec![] as Vec<Vec<usize>>, |mut acc, numbers| {
                if acc.is_empty() {
                    acc = vec![vec![]; numbers.len()];
                } else if acc.len() != numbers.len() {
                    return Err(anyhow!("Inconsistent number of columns in number grid."));
                }

                Ok(acc
                    .into_iter()
                    .zip(numbers)
                    .map(|(mut col_vec, &num)| {
                        col_vec.push(num);
                        col_vec
                    })
                    .collect::<Vec<Vec<usize>>>())
            })?;

        if operators.len() != math_numbers.len() {
            return Err(anyhow!(
                "Number of operators does not match number of tasks."
            ));
        }

        Ok(math_numbers
            .into_iter()
            .zip(operators)
            .map(|(numbers, operator)| MathTask { numbers, operator })
            .collect())
    }
}

impl FromStr for MathTasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();

        let operator_line = lines_iter
            .next_back()
            .ok_or(anyhow!("Failed to take operators."))?;

        let operators = operator_line
            .split_whitespace()
            .map(Operator::from_str)
            .collect::<Result<Vec<Operator>, _>>()?;

        let number_matrix = lines_iter.collect::<CharGrid>();

        Ok(MathTasks {
            char_grid: number_matrix,
            operators,
        })
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
            _ => Err(anyhow!("Unknown operator: {}", s)),
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
    }

    #[test]
    fn it_solves_normal() -> Result<(), anyhow::Error> {
        let math_tasks = INPUT.parse::<MathTasks>()?;
        assert_eq!(math_tasks.try_solve_normal()?, 4277556);
        Ok(())
    }
}

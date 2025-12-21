pub mod cephalopod;
pub mod human;

use std::str::{FromStr, Lines};

use anyhow::anyhow;

pub struct MathTasks<'a> {
    operators: Vec<Operator>,
    numbers: Lines<'a>,
}

impl<'a> MathTasks<'a> {
    fn len(&self) -> usize {
        self.operators.len()
    }

    pub fn try_solve<I>(&self) -> Result<usize, anyhow::Error>
    where
        I: MathParser<Term = Result<usize, anyhow::Error>> + From<Lines<'a>>,
    {
        let numbers = I::from(self.numbers.clone())
            .into_iter_assignment()
            .map(|it| it.collect::<Result<Vec<usize>, anyhow::Error>>());

        let tasks =
            numbers
                .into_iter()
                .zip(self.operators.iter().cloned())
                .map(|(numbers, operator)| {
                    Ok(MathTask {
                        numbers: numbers?,
                        operator,
                    })
                });

        Ok(tasks
            .map(|task: Result<MathTask, anyhow::Error>| Ok(task?.into_solution()))
            .collect::<Result<Vec<_>, anyhow::Error>>()?
            .into_iter()
            .sum())
    }
}

pub trait MathParser {
    type Term;
    type Equations: Iterator<Item = Self::Term>;

    fn into_iter_assignment(self) -> impl Iterator<Item = Self::Equations>;
}

impl<'a> TryFrom<&'a str> for MathTasks<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut lines_iter = s.lines();

        let operator_line = lines_iter
            .next_back()
            .ok_or(anyhow!("Failed to take operators."))?;

        let operators = operator_line
            .split_whitespace()
            .map(Operator::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let numbers = lines_iter;

        Ok(MathTasks { operators, numbers })
    }
}
struct MathTask {
    numbers: Vec<usize>,
    operator: Operator,
}

impl MathTask {
    fn into_solution(self) -> usize {
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

    use crate::math::{cephalopod::CephalopodNumberParser, human::HumanNumberParser};

    use super::*;
    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn it_parses() {
        let math_tasks: MathTasks = INPUT.try_into().expect("Failed to parse input");
        assert_eq!(math_tasks.len(), 4);
    }

    #[test]
    fn it_solves_normal() -> Result<(), anyhow::Error> {
        let math_tasks: MathTasks = INPUT.try_into().expect("Failed to parse input");
        assert_eq!(math_tasks.try_solve::<HumanNumberParser>()?, 4277556);
        Ok(())
    }

    #[test]
    fn it_solves_cephalopod() -> Result<(), anyhow::Error> {
        let math_tasks: MathTasks = INPUT.try_into().expect("Failed to parse input");
        assert_eq!(math_tasks.try_solve::<CephalopodNumberParser>()?, 3263827);
        Ok(())
    }
}

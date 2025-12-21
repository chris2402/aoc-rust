use std::{str::Lines, vec::IntoIter};

use crate::{math::MathParser, transpose::Transpose};

pub struct CephalopodNumberParser<'a>(Lines<'a>);

impl<'a> From<Lines<'a>> for CephalopodNumberParser<'a> {
    fn from(lines: Lines<'a>) -> Self {
        CephalopodNumberParser(lines)
    }
}

impl MathParser for CephalopodNumberParser<'_> {
    type Equations = IntoIter<Self::Term>;
    type Term = Result<usize, anyhow::Error>;

    fn into_iter_assignment(self) -> impl Iterator<Item = Self::Equations> {
        let transposed: Vec<String> = self
            .0
            .map(|l| l.chars())
            .transpose()
            .map(|chars| chars.into_iter().collect::<String>())
            .collect();

        transposed
            .into_iter()
            .map(|chars| chars.trim().to_string())
            .collect::<Vec<_>>()
            .split(|line| line.is_empty())
            .map(|group| {
                group
                    .iter()
                    .map(|line| line.parse::<usize>().map_err(anyhow::Error::from))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314";

    #[test]
    fn it_parses_to_math_tasks() -> Result<(), anyhow::Error> {
        let tasks = CephalopodNumberParser(INPUT.lines())
            .into_iter_assignment()
            .map(Iterator::collect::<Result<Vec<_>, anyhow::Error>>)
            .collect::<Result<Vec<Vec<usize>>, anyhow::Error>>()?;

        assert_eq!(tasks.len(), 4);
        assert_eq!(tasks[0], vec![1, 24, 356]);
        assert_eq!(tasks[1], vec![369, 248, 8]);
        assert_eq!(tasks[2], vec![32, 581, 175]);
        assert_eq!(tasks[3], vec![623, 431, 4]);
        Ok(())
    }
}

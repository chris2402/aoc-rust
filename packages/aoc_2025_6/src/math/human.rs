use std::{str::Lines, vec::IntoIter};

use crate::{math::MathParser, transpose::Transpose};

pub struct HumanNumberParser<'a>(Lines<'a>);

impl<'a> From<Lines<'a>> for HumanNumberParser<'a> {
    fn from(lines: Lines<'a>) -> Self {
        HumanNumberParser(lines)
    }
}

fn parse_term(s: &str) -> Result<usize, anyhow::Error> {
    s.parse::<usize>().map_err(anyhow::Error::from)
}

impl<'a> MathParser for HumanNumberParser<'a> {
    type Equations = IntoIter<Self::Term>;
    type Term = Result<usize, anyhow::Error>;

    fn into_iter_assignment(self) -> impl Iterator<Item = Self::Equations> {
        self.0
            .map(|l| {
                l.split_whitespace()
                    .map(parse_term as fn(&str) -> Self::Term)
            })
            .transpose()
            .map(|line| line.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314";

    #[test]
    fn it_reads_lines() -> Result<(), anyhow::Error> {
        let lines = HumanNumberParser(INPUT.lines())
            .into_iter_assignment()
            .map(Iterator::collect::<Result<Vec<_>, anyhow::Error>>)
            .collect::<Result<Vec<Vec<usize>>, anyhow::Error>>()?;

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], vec![123, 45, 6]);
        assert_eq!(lines[1], vec![328, 64, 98]);
        assert_eq!(lines[2], vec![51, 387, 215]);
        assert_eq!(lines[3], vec![64, 23, 314]);
        Ok(())
    }
}

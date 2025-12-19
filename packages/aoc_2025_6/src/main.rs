#![allow(unused)]
mod math;
mod number;

use math::MathTasks;
use number::CharGrid;

use std::{slice::Iter, str::FromStr, vec::IntoIter};

use anyhow::anyhow;
use array2d::Array2D;

fn main() {
    let input = std::fs::read_to_string("packages/aoc_2025_6/input.txt")
        .expect("Failed to read input file");
    // let result_1 = solve_part_1(&input).expect("Failed to solve part 1");
    // println!("Part 1: {}", result_1);
}

// fn try_iter<'a>(&'a self) -> IntoIter<Result<MathTask, anyhow::Error>> {
//     let Ok(tasks) = self
//         .number_matrix
//         .rows_iter()
//         .map(|row| {
//             row.collect::<String>()
//                 .split_whitespace()
//                 .map(|num| num.parse::<usize>())
//                 .collect::<Result<Vec<usize>, _>>()
//         })
//         .collect::<Result<Vec<Vec<usize>>, _>>()
//     else {
//         return vec![Err(anyhow!("Failed to parse numbers from matrix."))].into_iter();
//     };

//     let no_rows = self.number_matrix.num_rows();

//     self.operators
//         .iter()
//         .copied()
//         .enumerate()
//         .map(|(i, operator)| {
//             let Ok(numbers): Result<Vec<_>, _> = tasks
//                 .iter()
//                 .map(|row| {
//                     row.get(i)
//                         .map(usize::to_owned)
//                         .ok_or(anyhow!("Failed to get number from row"))
//                 })
//                 .collect()
//             else {
//                 return Err(anyhow!("Failed to collect numbers for task."));
//             };

//             Ok(MathTask { numbers, operator })
//         })
//         .collect::<Vec<_>>()
//         .into_iter()
// }

// }

// impl FromStr for MathTasks {
//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim();
//         let total_len = s.len();
//         let mut lines_iter = s.split('\n');

//         let operator_line = lines_iter
//             .next_back()
//             .ok_or(anyhow!("Failed to take operators."))?;

//         let operators = operator_line
//             .split_whitespace()
//             .map(Operator::from_str)
//             .collect::<Result<Vec<Operator>, _>>()?;

//         let line_len = operator_line.len();
//         let no_lines = total_len / (line_len + 1);

//         let char_iter = lines_iter.flat_map(|line| line.chars());

//         let x = char_iter.clone().count();
//         let number_matrix = Array2D::from_iter_row_major(char_iter, no_lines, line_len)?;

//         let column_split_indices = operator_line
//             .char_indices()
//             .skip(1) // first char is always an operator
//             .filter(|(_, c)| !c.is_whitespace())
//             .map(|(idx, _)| idx - 1)
//             .collect::<Vec<usize>>();

//         Ok(MathTasks {
//             number_matrix,
//             operators,
//             column_split_indices,
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use core::num;

    use anyhow::Ok;

    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn it_parses_number_grid() -> Result<(), anyhow::Error> {
        let mut lines = INPUT.lines();

        let operator_line = lines.next_back().unwrap(); // remove operator line

        let number_grid: CharGrid = lines.collect();

        assert_eq!(number_grid.num_rows(), 3);
        assert_eq!(number_grid.num_columns(), 15);
        assert_eq!(
            number_grid.get_nth_horizontal(0),
            Some("123 328  51 64 ".to_string())
        );
        assert_eq!(number_grid.get_nth_vertical(1), Some("24 ".to_string()));
        Ok(())
    }

    // #[test]
    // fn it_parses() {
    //     let math_tasks = INPUT.parse::<MathTasks>().expect("Failed to parse input");

    //     assert_eq!(math_tasks.len(), 4);
    // assert_eq!(
    //     math_tasks
    //         .try_iter()
    //         .collect::<Result<Vec<MathTask>, _>>()
    //         .expect("Failed to collect math tasks")
    //         .iter()
    //         .filter(|mt| matches!(mt.operator, Operator::Add))
    //         .count(),
    //     2
    // );
    // assert_eq!(
    //     math_tasks
    //         .try_iter()
    //         .collect::<Result<Vec<MathTask>, _>>()
    //         .expect("Failed to collect math tasks")
    //         .iter()
    //         .filter(|mt| matches!(mt.operator, Operator::Multiply))
    //         .count(),
    //     2
    // );
    // }

    // #[test]
    // fn it_solves_1() {
    //     let math_tasks = INPUT.parse::<MathTasks>().expect("Failed to parse input");

    //     let individual_results: Vec<usize> = math_tasks
    //         .try_iter()
    //         .flat_map(|mt_result| mt_result.map(|mt| mt.solve()))
    //         .collect();
    //     let result: usize = math_tasks.solve_all_1();
    //     assert_eq!(individual_results, vec![33210, 490, 4243455, 401]);
    //     assert_eq!(result, 4277556);
    // }
}

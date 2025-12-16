use std::str::FromStr;

use array2d::Array2D;

fn main() -> anyhow::Result<()> {
    let board = std::fs::read_to_string("packages/aoc_2025_4/input.txt")
        .expect("Failed to read input file")
        .parse::<Board>()?;

    let result_1 = solve_1(&board);
    println!("Part 1: {}", result_1);

    Ok(())
}

fn solve_1(board: &Board) -> usize {
    let mut count = 0;

    for position in board {
        if let Position(row, col, Square::Filled) = position {
            let filled_adjacent = board
                .adjacent_squares_of(row, col)
                .iter()
                .filter_map(|&&sq| if sq == Square::Filled { Some(()) } else { None })
                .count();
            if filled_adjacent < 4 {
                count += 1;
            }
        }
    }

    count
}

struct Position<'a>(usize, usize, &'a Square);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Filled,
}

impl TryFrom<char> for Square {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Square::Empty),
            '@' => Ok(Square::Filled),
            _ => Err(anyhow::anyhow!("Invalid character for Square: {}", c)),
        }
    }
}

struct Board(array2d::Array2D<Square>);

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(Square::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|d| Array2D::from_rows(&d).map_err(|e| anyhow::anyhow!(e)))?;

        Ok(Board(rows))
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = Position<'a>;

    type IntoIter = Positions<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Positions {
            board: self,
            row: 0,
            col: 0,
        }
    }
}

struct Positions<'a> {
    board: &'a Board,
    row: usize,
    col: usize,
}

impl<'a> Iterator for Positions<'a> {
    type Item = Position<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.board.0.num_rows() {
            return None;
        }

        let square = self
            .board
            .get_square(self.row, self.col)
            .map(|sq| Position(self.row, self.col, sq));

        self.col += 1;
        if self.col >= self.board.0.num_columns() {
            self.col = 0;
            self.row += 1;
        }

        square
    }
}

impl Board {
    fn get_square(&self, row: usize, col: usize) -> Option<&Square> {
        // Return None if out of bounds
        if row >= self.0.num_rows() || col >= self.0.num_columns() {
            None
        } else {
            self.0.get(row, col)
        }
    }

    fn adjacent_squares_of(&self, row: usize, col: usize) -> Vec<&Square> {
        let mut adjacents = Vec::new();

        let top_row = row.wrapping_sub(1); // Overflow to usize::MAX if row is 0
        let bottom_row = row + 1;
        let left_col = col.wrapping_sub(1); // Overflow to usize::MAX if col is 0
        let right_col = col + 1;

        // Since 0 - 1 wraps to usize::MAX, those will be out of bounds and return None
        adjacents.push(self.get_square(top_row, left_col).unwrap_or(&Square::Empty));
        adjacents.push(self.get_square(top_row, col).unwrap_or(&Square::Empty));
        adjacents.push(
            self.get_square(top_row, right_col)
                .unwrap_or(&Square::Empty),
        );
        adjacents.push(self.get_square(row, left_col).unwrap_or(&Square::Empty));
        adjacents.push(self.get_square(row, right_col).unwrap_or(&Square::Empty));
        adjacents.push(
            self.get_square(bottom_row, left_col)
                .unwrap_or(&Square::Empty),
        );
        adjacents.push(self.get_square(bottom_row, col).unwrap_or(&Square::Empty));
        adjacents.push(
            self.get_square(bottom_row, right_col)
                .unwrap_or(&Square::Empty),
        );
        adjacents
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn it_parses() -> Result<(), anyhow::Error> {
        let board: Board = INPUT.parse()?;

        assert_eq!(board.get_square(0, 0), Some(&Square::Empty));
        assert_eq!(board.get_square(0, 1), Some(&Square::Empty));
        assert_eq!(board.get_square(0, 2), Some(&Square::Filled));
        assert_eq!(board.get_square(0, 3), Some(&Square::Filled));
        assert_eq!(board.get_square(3, 1), Some(&Square::Empty));
        assert_eq!(board.get_square(1, 0), Some(&Square::Filled));
        Ok(())
    }
    #[test]
    fn it_solves_1() -> Result<(), anyhow::Error> {
        let board: Board = INPUT.parse()?;
        let result = solve_1(&board);
        assert_eq!(result, 13);
        Ok(())
    }
}

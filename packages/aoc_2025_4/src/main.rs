fn main() {
    let input = std::fs::read_to_string("packages/aoc_2025_4/input.txt")
        .expect("Failed to read input file");
    todo!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Filled,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Square::Empty,
            '@' => Square::Filled,
            _ => panic!("Invalid character for Square: {}", c),
        }
    }
}

type Board = array2d::Array2D<Square>;

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
        let input = INPUT
            .lines()
            .map(|l| l.chars().map(Square::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let board = Board::from_rows(&input)?;
        assert_eq!(board[(0, 0)], Square::Empty);
        assert_eq!(board[(0, 1)], Square::Empty);
        assert_eq!(board[(0, 2)], Square::Filled);
        assert_eq!(board[(0, 3)], Square::Filled);
        assert_eq!(board[(3, 1)], Square::Empty);
        assert_eq!(board[(1, 0)], Square::Filled);
        Ok(())
    }
}

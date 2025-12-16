use std::str::FromStr;

use array2d::Array2D;

fn main() -> anyhow::Result<()> {
    let floor = std::fs::read_to_string("packages/aoc_2025_4/input.txt")
        .expect("Failed to read input file")
        .parse::<Floor>()?;

    let result_1 = solve_1(&floor);
    println!("Part 1: {}", result_1);
    let result_2 = solve_2(&mut floor.clone());
    println!("Part 2: {}", result_2);

    Ok(())
}

fn solve_1(floor: &Floor) -> usize {
    let mut total_removed = 0;

    for position in floor {
        if let Tile(row, col, TileContent::RollOfPaper) = position {
            let adjacents_with_roll = floor
                .adjacent_tile_content(row, col)
                .iter()
                .filter_map(|&&sq| {
                    if sq == TileContent::RollOfPaper {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();
            if adjacents_with_roll < 4 {
                total_removed += 1;
            }
        }
    }

    total_removed
}

fn solve_2(floor: &mut Floor) -> usize {
    let mut total_removed = 0;

    // Continue until we've initialized flag, and we've set it to false in a full pass
    loop {
        let mut tiles_with_removed_roll = vec![];
        for tile in &*floor {
            if let Tile(row, col, TileContent::RollOfPaper) = tile {
                let adjacents_with_roll = floor
                    .adjacent_tile_content(row, col)
                    .into_iter()
                    .filter(|content| TileContent::is_roll(content))
                    .count();

                if adjacents_with_roll < 4 {
                    tiles_with_removed_roll.push((row, col));
                }
            }
        }
        if tiles_with_removed_roll.is_empty() {
            break;
        }
        total_removed += tiles_with_removed_roll.len();
        for &(row, col) in tiles_with_removed_roll.iter() {
            floor.remove(row, col);
        }
    }

    total_removed
}

struct Tile<'a>(usize, usize, &'a TileContent);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileContent {
    Nothing,
    RollOfPaper,
}

impl TileContent {
    fn is_roll(&self) -> bool {
        matches!(self, TileContent::RollOfPaper)
    }
}

impl TryFrom<char> for TileContent {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(TileContent::Nothing),
            '@' => Ok(TileContent::RollOfPaper),
            _ => Err(anyhow::anyhow!("Invalid character for Square: {}", c)),
        }
    }
}

#[derive(Clone)]
struct Floor(array2d::Array2D<TileContent>);

impl FromStr for Floor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(TileContent::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|d| Array2D::from_rows(&d).map_err(|e| anyhow::anyhow!(e)))?;

        Ok(Floor(rows))
    }
}

impl<'a> IntoIterator for &'a Floor {
    type Item = Tile<'a>;

    type IntoIter = Tiles<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Tiles {
            floor: self,
            row: 0,
            col: 0,
        }
    }
}

struct Tiles<'a> {
    floor: &'a Floor,
    row: usize,
    col: usize,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = Tile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.floor.0.num_rows() {
            return None;
        }

        let square = self
            .floor
            .get_tile_content(self.row, self.col)
            .map(|sq| Tile(self.row, self.col, sq));

        self.col += 1;
        if self.col >= self.floor.0.num_columns() {
            self.col = 0;
            self.row += 1;
        }

        square
    }
}

impl Floor {
    fn get_tile_content(&self, row: usize, col: usize) -> Option<&TileContent> {
        // Return None if out of bounds
        if row >= self.0.num_rows() || col >= self.0.num_columns() {
            None
        } else {
            self.0.get(row, col)
        }
    }

    fn adjacent_tile_content(&self, row: usize, col: usize) -> Vec<&TileContent> {
        let mut adjacents = Vec::new();

        let top_row = row.wrapping_sub(1); // Overflow to usize::MAX if row is 0
        let bottom_row = row + 1;
        let left_col = col.wrapping_sub(1); // Overflow to usize::MAX if col is 0
        let right_col = col + 1;

        // Since 0 - 1 wraps to usize::MAX, those will be out of bounds and return None
        adjacents.push(
            self.get_tile_content(top_row, left_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(top_row, col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(top_row, right_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(row, left_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(row, right_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(bottom_row, left_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(bottom_row, col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents.push(
            self.get_tile_content(bottom_row, right_col)
                .unwrap_or(&TileContent::Nothing),
        );
        adjacents
    }

    fn remove(&mut self, row: usize, col: usize) {
        self.0
            .set(row, col, TileContent::Nothing)
            .expect("Failed to remove square");
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
        let board: Floor = INPUT.parse()?;

        assert_eq!(board.get_tile_content(0, 0), Some(&TileContent::Nothing));
        assert_eq!(board.get_tile_content(0, 1), Some(&TileContent::Nothing));
        assert_eq!(
            board.get_tile_content(0, 2),
            Some(&TileContent::RollOfPaper)
        );
        assert_eq!(
            board.get_tile_content(0, 3),
            Some(&TileContent::RollOfPaper)
        );
        assert_eq!(board.get_tile_content(3, 1), Some(&TileContent::Nothing));
        assert_eq!(
            board.get_tile_content(1, 0),
            Some(&TileContent::RollOfPaper)
        );
        Ok(())
    }

    #[test]
    fn it_solves_1() -> Result<(), anyhow::Error> {
        let board: Floor = INPUT.parse()?;
        let result = solve_1(&board);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn it_solves_2() -> Result<(), anyhow::Error> {
        let mut board: Floor = INPUT.parse()?;
        let result = solve_2(&mut board);
        assert_eq!(result, 43);
        Ok(())
    }
}

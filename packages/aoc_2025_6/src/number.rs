use std::{path::Iter, str::FromStr};

use array2d::Array2D;

pub struct CharGrid(Array2D<char>);

impl CharGrid {
    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }

    pub fn get_nth_horizontal(&self, n: usize) -> Option<String> {
        if n >= self.num_rows() {
            None
        } else {
            self.0.row_iter(n).ok().map(|c| c.collect::<String>())
        }
    }

    pub fn get_nth_vertical(&self, n: usize) -> Option<String> {
        if n >= self.num_columns() {
            None
        } else {
            self.0
                .rows_iter()
                .map(|mut row| row.nth(n))
                .collect::<Option<String>>()
        }
    }

    pub fn vertical_iter(&self) -> impl Iterator<Item = String> + '_ {
        VerticalLineIterator {
            grid: self,
            current_col: 0,
        }
    }

    pub fn horizontal_iter(&self) -> impl Iterator<Item = String> + '_ {
        HorizontalLineIterator {
            grid: self,
            current_row: 0,
        }
    }
}

impl<'a> FromIterator<&'a str> for CharGrid {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let rows: Vec<Vec<char>> = iter
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        CharGrid(Array2D::from_rows(&rows).expect("Failed to create CharGrid from lines"))
    }
}

impl FromStr for CharGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        Ok(lines.collect())
    }
}

struct VerticalLineIterator<'a> {
    grid: &'a CharGrid,
    current_col: usize,
}

impl Iterator for VerticalLineIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid
            .get_nth_vertical(self.current_col)
            .inspect(|line| {
                self.current_col += 1;
            })
    }
}

struct HorizontalLineIterator<'a> {
    grid: &'a CharGrid,
    current_row: usize,
}

impl Iterator for HorizontalLineIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid
            .get_nth_horizontal(self.current_row)
            .inspect(|line| {
                self.current_row += 1;
            })
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Ok;

    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
";

    #[test]
    fn lines_collect_to_number_grid() -> Result<(), anyhow::Error> {
        let number_grid: CharGrid = INPUT.lines().collect();

        assert_eq!(number_grid.num_rows(), 3);
        assert_eq!(number_grid.num_columns(), 15);
        assert_eq!(
            number_grid.get_nth_horizontal(0),
            Some("123 328  51 64 ".to_string())
        );
        assert_eq!(number_grid.get_nth_vertical(1), Some("24 ".to_string()));
        Ok(())
    }

    #[test]
    fn vertical_line_iterator_works() {
        let number_grid: CharGrid = INPUT.lines().collect();
        let mut vertical_iter = number_grid.vertical_iter();
        assert_eq!(vertical_iter.next(), Some("1  ".to_string()));
        assert_eq!(vertical_iter.next(), Some("24 ".to_string()));
        assert_eq!(vertical_iter.next(), Some("356".to_string()));
        assert_eq!(vertical_iter.next(), Some("   ".to_string()));
    }

    #[test]
    fn horizontal_line_iterator_works() {
        let number_grid: CharGrid = INPUT.lines().collect();
        let mut horizontal_iter = number_grid.horizontal_iter();
        assert_eq!(horizontal_iter.next(), Some("123 328  51 64 ".to_string()));
        assert_eq!(horizontal_iter.next(), Some(" 45 64  387 23 ".to_string()));
        assert_eq!(horizontal_iter.next(), Some("  6 98  215 314".to_string()));
        assert_eq!(horizontal_iter.next(), None);
    }
}

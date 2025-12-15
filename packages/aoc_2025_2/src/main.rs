use std::{
    iter::Map,
    str::{FromStr, Split},
};

fn main() {
    let input = std::fs::read_to_string("packages/aoc_2025_2/input.txt")
        .expect("Failed to read input file");
    let ranges: ProductIdRanges = input.as_str().into();

    let result = ranges.solve_1().expect("Failed to solve part 1");
    println!("day1: {}", result);
    let result = ranges.solve_2().expect("Failed to solve part 2");
    println!("day2: {}", result);
}

trait ProductId {
    fn is_invalid_1(&self) -> bool;
    fn is_invalid_2(&self) -> bool;
}

impl ProductId for isize {
    fn is_invalid_1(&self) -> bool {
        let id_text: String = self.to_string();
        if !id_text.len().is_multiple_of(2) {
            return false;
        }
        let index = id_text.len() / 2;
        let part1 = &id_text[..index];
        let part2 = &id_text[index..];
        part1 == part2
    }

    fn is_invalid_2(&self) -> bool {
        // number lower than 10 cannot have repeating patterns
        if *self < 10 {
            return false;
        }

        let n = self.ilog10() + 1;
        for k in (1..n).filter(|k| n.is_multiple_of(*k)) {
            // b is the leading part; 10 to the power of n-k shifts right k digits
            let b = self / 10_isize.pow(n - k);

            let power_k = 10_isize.pow(k);
            let power_n = 10_isize.pow(n);
            let multiplier = (power_n - 1) / (power_k - 1);
            if b * multiplier == *self {
                return true;
            }
        }
        false
    }
}

struct ProductIdRanges<'a>(&'a str);

type ProductIdRangeIter<'a> =
    Map<Split<'a, &'a str>, fn(&'a str) -> Result<ProductIdRange, anyhow::Error>>;

impl<'a> IntoIterator for &ProductIdRanges<'a> {
    type Item = Result<ProductIdRange, anyhow::Error>;
    type IntoIter = ProductIdRangeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.trim().split(",").map(|line| line.parse())
    }
}

impl<'a> From<&'a str> for ProductIdRanges<'a> {
    fn from(s: &'a str) -> Self {
        ProductIdRanges(s)
    }
}

impl ProductIdRanges<'_> {
    fn solve_1(&self) -> Result<isize, anyhow::Error> {
        self.into_iter().try_fold(0_isize, |acc_sum, range_result| {
            let current_range_sum = range_result?
                .into_iter()
                .filter(|id| id.is_invalid_1())
                .sum::<isize>();
            Ok(acc_sum + current_range_sum)
        })
    }

    fn solve_2(&self) -> Result<isize, anyhow::Error> {
        self.into_iter().try_fold(0_isize, |acc_sum, range_result| {
            let current_range_sum = range_result?
                .into_iter()
                .filter(|id| id.is_invalid_2())
                .sum::<isize>();
            Ok(acc_sum + current_range_sum)
        })
    }
}

struct ProductIdRange(std::ops::RangeInclusive<isize>);

impl IntoIterator for ProductIdRange {
    type Item = isize;
    type IntoIter = std::ops::RangeInclusive<isize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

impl FromStr for ProductIdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("-");
        let start = lines
            .next()
            .ok_or(anyhow::anyhow!("Missing start of range"))?
            .parse::<isize>()?;
        let end = lines
            .next()
            .ok_or(anyhow::anyhow!("Missing end of range"))?
            .parse::<isize>()?;
        Ok(ProductIdRange(start..=end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example() -> String {
        String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        )
    }

    #[test]
    fn it_parses_a_range() -> Result<(), anyhow::Error> {
        let _example = "100-200";
        let _range: ProductIdRange = _example.parse()?;
        assert_eq!(100, *_range.0.start());
        assert_eq!(200, *_range.0.end());
        Ok(())
    }

    #[test]
    fn it_parses_many_ranges() {
        let example = load_example();
        let ranges: ProductIdRanges = example.as_str().into();
        assert_eq!(11, ranges.into_iter().count())
    }

    #[test]
    fn it_solves_part_1() -> Result<(), anyhow::Error> {
        let examples = load_example();
        let ranges: ProductIdRanges = examples.as_str().into();
        let actual = ranges.solve_1()?;
        let expected = 1227775554;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_solves_part_2() -> Result<(), anyhow::Error> {
        let examples = load_example();
        let ranges: ProductIdRanges = examples.as_str().into();

        let actual = ranges.solve_2()?;
        let expected = 4174379265;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_finds_invalid_id_part1() {
        let example = 100100_isize;
        assert!(example.is_invalid_1());
    }

    #[test]
    fn it_finds_invalid_id_part2() {
        let example = 1111111_isize;
        assert!(example.is_invalid_2());
    }

    #[test]
    fn it_can_abuse_split() {
        let example = "11111";
        let parts: Vec<&str> = example.split("1").collect();
        assert_eq!(6, parts.len());
    }
}

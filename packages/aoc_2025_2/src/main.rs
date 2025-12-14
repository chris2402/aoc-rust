use std::str::FromStr;

fn main() {
    let _input = std::fs::read_to_string("packages/aoc_2025_2/input.txt")
        .expect("Failed to read input file");
    let ranges: MyRanges = _input.parse().unwrap();
    let result = ranges.solve_1();
    println!("Result: {}", result);
}

struct MyRange(std::ops::RangeInclusive<isize>);
struct MyRanges(Vec<MyRange>);

impl MyRanges {
    fn solve_1(&self) -> isize {
        self.0
            .iter()
            .flat_map(|range| range.iter_invalid_product_ids())
            .collect::<Vec<isize>>()
            .iter()
            .sum()
    }
}

impl MyRange {
    fn iter_invalid_product_ids(&self) -> Vec<isize> {
        let mut invalid_product_ids = Vec::new();
        for id in self.0.clone() {
            if is_invalid_id(id) {
                invalid_product_ids.push(id);
            }
        }
        invalid_product_ids
    }
}

fn is_invalid_id(id: isize) -> bool {
    let id_text: String = id.to_string();
    if id_text.len() % 2 != 0 {
        return false;
    }
    let index = id_text.len() / 2;
    let part1 = &id_text[..index];
    let part2 = &id_text[index..];
    return part1 == part2;
}

impl FromStr for MyRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("-");
        let start = lines.next().unwrap().parse::<isize>()?;
        let end = lines.next().unwrap().parse::<isize>()?;
        Ok(MyRange(start..=end))
    }
}

impl FromStr for MyRanges {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<MyRange> = s
            .trim()
            .split(",")
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(MyRanges(ranges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example() -> String {
        std::fs::read_to_string("example.txt").expect("Failed to read example file")
    }

    #[test]
    fn it_works() {
        let _example = "100-200";
        let _range: MyRange = _example.parse().unwrap();
        assert_eq!(100, *_range.0.start());
        assert_eq!(200, *_range.0.end());
    }

    #[test]
    fn my_ranges_works() {
        let example = load_example();
        let ranges: MyRanges = example.parse().unwrap();
        assert_eq!(11, ranges.0.len())
    }

    #[test]
    fn test_part_1() {
        let ranges: MyRanges = load_example().parse().unwrap();
        let actual = ranges.solve_1();
        let expected = 1227775554;
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_is_invalid_id_part1() {
        let example = 100100_isize;
        assert!(is_invalid_id(example));
    }
}

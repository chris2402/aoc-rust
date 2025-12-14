use std::str::FromStr;

fn main() {
    let _input = std::fs::read_to_string("packages/aoc_2025_2/input.txt")
        .expect("Failed to read input file");
    let ranges: MyRanges = _input.parse().unwrap();
    let result = ranges.solve_1();
    println!("day1: {}", result);
    let result = ranges.solve_2();
    println!("day2: {}", result);
}

struct MyRange(std::ops::RangeInclusive<isize>);
struct MyRanges(Vec<MyRange>);

impl MyRanges {
    fn solve_1(&self) -> isize {
        self.0
            .iter()
            .flat_map(|range| range.iter_invalid_product_ids_part(is_invalid_id_part_1))
            .collect::<Vec<isize>>()
            .iter()
            .sum()
    }

    fn solve_2(&self) -> isize {
        self.0
            .iter()
            .flat_map(|range| range.iter_invalid_product_ids_part(is_invalid_id_part_2))
            .collect::<Vec<isize>>()
            .iter()
            .sum()
    }
}

impl MyRange {
    fn iter_invalid_product_ids_part(&self, pred: fn(isize) -> bool) -> Vec<isize> {
        let mut invalid_product_ids = Vec::new();
        for id in self.0.clone() {
            if pred(id) {
                invalid_product_ids.push(id);
            }
        }
        invalid_product_ids
    }
}

fn is_invalid_id_part_1(id: isize) -> bool {
    let id_text: String = id.to_string();
    if !id_text.len().is_multiple_of(2) {
        return false;
    }
    let index = id_text.len() / 2;
    let part1 = &id_text[..index];
    let part2 = &id_text[index..];
    part1 == part2
}

fn is_invalid_id_part_2(_id: isize) -> bool {
    // Placeholder for part 2 logic
    let id_text: String = _id.to_string();
    let n = id_text.len();
    let range = (1..=(n / 2)).filter(|x| n.is_multiple_of(*x)).rev();
    for r in range {
        let split = &id_text[..r];
        let parts = id_text.split(split).collect::<String>();
        if parts.is_empty() {
            return true;
        }
    }
    false
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
    fn test_part_2() {
        let ranges: MyRanges = load_example().parse().unwrap();
        let actual = ranges.solve_2();
        let expected = 4174379265;
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_is_invalid_id_part1() {
        let example = 100100_isize;
        assert!(is_invalid_id_part_1(example));
    }

    #[test]
    fn test_is_invalid_id_part2() {
        let example = 1111111_isize;
        assert!(is_invalid_id_part_2(example));
    }

    #[test]
    fn test_if_we_can_abuse_split() {
        let example = "11111";
        let parts: Vec<&str> = example.split("1").collect();
        assert_eq!(6, parts.len());
    }
}

use std::{ops::RangeInclusive, slice::Iter, str::FromStr, vec::IntoIter};

fn main() {
    let _input = std::fs::read_to_string("packages/aoc_2025_5/input.txt")
        .expect("Failed to read input file");

    let InputTuple(mut fresh_id_ranges, ingredient_ids) =
        _input.parse::<InputTuple>().expect("Failed to parse input");

    println!("Length of fresh id ranges: {}", fresh_id_ranges.len());
    fresh_id_ranges.merge_ranges();
    println!(
        "Length of merged fresh id ranges: {}",
        fresh_id_ranges.len()
    );
    println!("Length of ingredient ids: {}", ingredient_ids.len());

    let result_1 = solve_part_1(&_input).expect("Failed to solve part 1");
    println!("Part 1: {}", result_1);

    let result_2 = solve_part_2(&_input).expect("Failed to solve part 2");
    println!("Part 2: {}", result_2);
}

fn solve_part_1(_input: &str) -> Result<usize, anyhow::Error> {
    let InputTuple(mut fresh_id_ranges, mut ingredient_ids) = _input.parse::<InputTuple>()?;
    fresh_id_ranges.merge_ranges();

    let total_ingredient_ids = ingredient_ids.len();
    for fresh_id_range in fresh_id_ranges.into_iter() {
        ingredient_ids.retain(|id| !id.in_range(fresh_id_range));
    }

    Ok(total_ingredient_ids - ingredient_ids.len())
}

fn solve_part_2(_input: &str) -> Result<usize, anyhow::Error> {
    let InputTuple(mut fresh_id_ranges, _) = _input.parse::<InputTuple>()?;
    fresh_id_ranges.merge_ranges();
    Ok(fresh_id_ranges
        .into_iter()
        .map(FreshIngredientIdRange::len)
        .sum())
}

struct InputTuple(FreshIngredientIdRanges, IngredientIds);

impl FromStr for InputTuple {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges, ids) = s.split_once("\n\n").ok_or_else(|| {
            anyhow::anyhow!("Input does not contain two sections separated by a blank line")
        })?;
        Ok(InputTuple(ranges.parse()?, ids.parse()?))
    }
}

struct FreshIngredientIdRanges(Vec<FreshIngredientIdRange>);

impl FreshIngredientIdRanges {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn merge_ranges(&mut self) {
        self.0.sort_by(|a, b| a.0.start().cmp(b.0.start()));

        let mut merged_ranges: Vec<FreshIngredientIdRange> = Vec::new();

        'ranges: for range in &self.0 {
            for merged in merged_ranges.iter_mut() {
                if range.is_overlapping(merged) {
                    let new_start = std::cmp::min(*range.0.start(), *merged.0.start());
                    let new_end = std::cmp::max(*range.0.end(), *merged.0.end());
                    merged.0 = new_start..=new_end;
                    continue 'ranges;
                }
            }

            merged_ranges.push(FreshIngredientIdRange(range.0.clone()));
        }

        self.0 = merged_ranges;
    }
}

impl<'a> IntoIterator for &'a FreshIngredientIdRanges {
    type Item = &'a FreshIngredientIdRange;
    type IntoIter = Iter<'a, FreshIngredientIdRange>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromStr for FreshIngredientIdRanges {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner_iter = s
            .trim()
            .lines()
            .map(FreshIngredientIdRange::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FreshIngredientIdRanges(inner_iter))
    }
}

struct FreshIngredientIdRange(RangeInclusive<IngredientId>);

impl FreshIngredientIdRange {
    fn is_overlapping(&self, other: &FreshIngredientIdRange) -> bool {
        self.0.end() >= other.0.start() && self.0.start() <= other.0.start()
            || other.0.end() >= self.0.start() && other.0.start() <= self.0.start()
    }

    fn len(&self) -> usize {
        let start = *self.0.start();
        let end = *self.0.end();
        if end < start {
            0
        } else {
            (end.0 - start.0) + 1
        }
    }
}

impl FromStr for FreshIngredientIdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid range format: {}", s))?;

        let start: IngredientId = start_str.parse()?;
        let end: IngredientId = end_str.parse()?;
        Ok(FreshIngredientIdRange(start..=end))
    }
}
struct IngredientIds(Vec<IngredientId>);

impl IngredientIds {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&IngredientId) -> bool,
    {
        self.0.retain(f);
    }
}

impl IntoIterator for IngredientIds {
    type Item = IngredientId;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for IngredientIds {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let inner_iter = _s
            .trim()
            .lines()
            .map(IngredientId::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IngredientIds(inner_iter))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct IngredientId(usize);

impl IngredientId {
    fn in_range(&self, range: &FreshIngredientIdRange) -> bool {
        range.0.contains(self)
    }
}

impl FromStr for IngredientId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: usize = s
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse IngredientId: {}", e))?;
        Ok(IngredientId(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn it_parses() -> Result<(), anyhow::Error> {
        let InputTuple(fresh_id_ranges, ingredient_ids) = INPUT.parse::<InputTuple>()?;

        assert_eq!(fresh_id_ranges.len(), 4);
        assert_eq!(ingredient_ids.len(), 6);

        let mut fresh_ids_iter = fresh_id_ranges.into_iter();
        let mut ingredient_ids_iter = ingredient_ids.into_iter();

        let bad_ingredient_id = ingredient_ids_iter
            .next()
            .ok_or(anyhow::anyhow!("Not enough ingredient ids"))?;

        let fresh_id_range = fresh_ids_iter
            .next()
            .ok_or(anyhow::anyhow!("Not enough fresh id ranges"))?;

        assert!(!bad_ingredient_id.in_range(fresh_id_range));
        assert!(IngredientId(5).in_range(fresh_id_range));
        assert!(IngredientId(3).in_range(fresh_id_range));
        Ok(())
    }

    #[test]
    fn it_correctly_filters_ingredient_ids() -> Result<(), anyhow::Error> {
        let InputTuple(mut fresh_id_ranges, mut ingredient_ids) = INPUT.parse::<InputTuple>()?;

        fresh_id_ranges.merge_ranges();

        for fresh_id_range in fresh_id_ranges.into_iter() {
            ingredient_ids.retain(|id| !id.in_range(fresh_id_range));
        }

        let remaining_ids: Vec<IngredientId> = ingredient_ids.into_iter().collect();
        let expected_ids = vec![IngredientId(1), IngredientId(8), IngredientId(32)];
        assert_eq!(remaining_ids, expected_ids);
        Ok(())
    }

    #[test]
    fn it_retains_correctly() -> Result<(), anyhow::Error> {
        let range = FreshIngredientIdRange(IngredientId(1)..=IngredientId(5));
        assert!(!IngredientId(0).in_range(&range));
        assert!(IngredientId(3).in_range(&range));
        assert!(IngredientId(2).in_range(&range));
        assert!(IngredientId(4).in_range(&range));
        assert!(IngredientId(5).in_range(&range));
        assert!(!IngredientId(6).in_range(&range));
        assert!(!IngredientId(7).in_range(&range));
        Ok(())
    }

    #[test]
    fn it_merges_ranges_correctly() -> Result<(), anyhow::Error> {
        let InputTuple(mut id_ranges, _) = INPUT.parse::<InputTuple>()?;
        assert_eq!(id_ranges.len(), 4);
        id_ranges.merge_ranges();
        assert_eq!(id_ranges.len(), 2);

        Ok(())
    }

    #[test]
    fn it_solves_part_1() -> Result<(), anyhow::Error> {
        let result = solve_part_1(INPUT)?;
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn it_solves_part_2() -> Result<(), anyhow::Error> {
        let result = solve_part_2(INPUT)?;
        assert_eq!(result, 14);
        Ok(())
    }
}

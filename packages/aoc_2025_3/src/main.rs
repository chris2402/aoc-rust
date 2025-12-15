fn main() {
    let input = std::fs::read_to_string("packages/aoc_2025_3/input.txt")
        .expect("Failed to read input file");

    println!("{}", solve(&input, alg_1));
    println!("{}", solve(&input, alg_2));
}

fn solve(input: &str, f: fn(&str) -> usize) -> usize {
    input.lines().map(f).sum()
}
fn alg_2(line: &str) -> usize {
    let mut digits = iter_digits(line);
    let mut arr: Vec<usize> = digits.by_ref().take(12).collect();

    arr.reverse();
    for d in digits.by_ref() {
        insert(arr.as_mut_slice(), d);
    }

    arr.iter()
        .zip((0..).map(|i| 10_usize.pow(i)))
        .map(|(digit, pow)| digit * pow)
        .sum()
}

fn insert(arr: &mut [usize], element: usize) {
    if arr.is_empty() {
        return; // base case
    }

    let curr = arr[0];
    if element > curr {
        arr[0] = element;
        insert(&mut arr[1..], curr);
    } else if squeeze(arr) {
        arr[0] = element;
    }
}

fn squeeze(arr: &mut [usize]) -> bool {
    // We move backwards through the array - high to low
    let mut it = arr.iter_mut().rev().peekable();
    let mut squeezed = false;
    while let Some(left) = it.next() {
        // The only exit from this loop is when there is no right element
        let Some(right) = it.peek() else {
            return squeezed;
        };

        // If we found a place to squeeze, or have squeezed already, we can just move the left to right
        squeezed = squeezed || &left < right;
        if squeezed {
            *left = **right;
        };
    }

    false // Will only reach here if one or less element is left
}

fn alg_1(line: &str) -> usize {
    let (first, second) = iter_digits(line).fold((0, 0), |(first, second), curr| {
        // There are more elements, so we can move the larger to first, and set second to minimum of curr!
        let (first, second) = if second > first {
            (second, 0)
        } else {
            (first, second)
        };

        // Now compare the current wiht the second
        if curr >= second {
            (first, curr)
        } else {
            (first, second)
        }
    });

    // Combine into two-digit numbers for the line
    first * 10 + second
}

fn iter_digits(line: &str) -> std::iter::Map<std::str::Chars<'_>, impl FnMut(char) -> usize> {
    line.chars().map(|c| c.to_digit(10).unwrap() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn it_solves_1_one_line() {
        let mut lines = INPUT.lines();
        assert_eq!(solve(lines.next().unwrap(), alg_1), 98);
        assert_eq!(solve(lines.next().unwrap(), alg_1), 89);
        assert_eq!(solve(lines.next().unwrap(), alg_1), 78);
        assert_eq!(solve(lines.next().unwrap(), alg_1), 92);
    }
    #[test]
    fn it_solves_1() {
        let result = solve(INPUT, alg_1);
        assert_eq!(result, 357);
    }

    #[test]
    fn it_solves_2_one_line() {
        let mut lines = INPUT.lines();
        assert_eq!(solve(lines.next().unwrap(), alg_2), 987654321111);
        assert_eq!(solve(lines.next().unwrap(), alg_2), 811111111119);
        assert_eq!(solve(lines.next().unwrap(), alg_2), 434234234278);
        assert_eq!(solve(lines.next().unwrap(), alg_2), 888911112111);
    }
    #[test]
    fn it_solves_2() {
        let result = solve(INPUT, alg_2);
        assert_eq!(result, 3121910778619);
    }
}

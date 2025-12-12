use anyhow::Result;
use crate::utils;

pub fn solve() -> Result<()> {
// Load your input file.
	let input = utils::load_input(2025, 5)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 5 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut lines = input.lines();
    let mut ranges = Vec::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push(start..=end);
    }

    let mut count = 0;
    for line in lines {
        let num: u64 = line.parse().unwrap();
        if ranges.iter().any(|r| r.contains(&num)) {
            count += 1;
        }
    }

	Ok(count)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut lines = input.lines();
    let mut ranges = Vec::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push(start..=end);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut total_covered = 0;
    let mut prev_end = 0;

    for range in ranges {
        let start = *range.start();
        let end = *range.end();

        let start = start.max(prev_end + 1);
        if start <= end {
            total_covered += end - start + 1;
            prev_end = end;
        }
    }

	Ok(total_covered)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_solve_part1() -> Result<()> {
        let result = solve_part1(TEST_INPUT)?;
        assert_eq!(result.to_string(), "3");
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> Result<()> {
        let result = solve_part2(TEST_INPUT)?;
        assert_eq!(result.to_string(), "14");
        Ok(())
    }
}
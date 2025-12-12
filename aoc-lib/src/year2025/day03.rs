use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input_lines(2025, 3)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 3 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(input: &Vec<String>) -> Result<impl std::fmt::Display> {
    let mut total: u64 = 0;

    for line in input {
        let line = line.trim().as_bytes();

        // Do not include last character
        let first_loc = find_first_largest(&line[..line.len() - 1]);
        let second_loc = find_first_largest(&line[first_loc + 1..]) + first_loc + 1;

        total += 10 * u64::from(line[first_loc] - b'0') + u64::from(line[second_loc] - b'0');

    }


    Ok(total)
}

fn solve_part2(input: &Vec<String>) -> Result<impl std::fmt::Display> {
    const N: usize = 12;
    let mut total: u64 = 0;

    for line in input {
        let line = line.trim().as_bytes();
        let mut line_joltage = 0;
        let mut start = 0;

        for i in 0..N {
            let loc = find_first_largest(&line[start..=(line.len() - N + i)]) + start;
            line_joltage = line_joltage * 10 + u64::from(line[loc] - b'0');
            start = loc + 1;
        }


        total += line_joltage;
    }

    Ok(total)
}

fn find_first_largest(bytes: &[u8]) -> usize {
    let mut max_loc = 0;
    for i in 1..bytes.len() {
        if bytes[i] > bytes[max_loc] {
            max_loc = i;
        }
    }
    max_loc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() -> Result<()> {
        let input = vec![
            String::from("987654321111111\n"),
            String::from("811111111111119\n"),
            String::from("234234234234278\n"),
            String::from("818181911112111\n"),
        ];

        let result = solve_part1(&input)?;
        assert_eq!(result.to_string(), "357");

        Ok(())
    }

    #[test]
        fn test_solve_part2() -> Result<()> {
            let input = vec![
                String::from("987654321111111\n"),
                String::from("811111111111119\n"),
                String::from("234234234234278\n"),
                String::from("818181911112111\n"),
            ];

            let result = solve_part2(&input)?;
            assert_eq!(result.to_string(), "3121910778619");

            Ok(())
        }
}
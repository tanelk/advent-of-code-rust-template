use anyhow::Result;
use crate::utils;


pub fn solve() -> Result<()> {
	let input = utils::load_input(2025, 4)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 4 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let input: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let X = input.len();
    let Y = input[0].len();
    let mut count = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for x in 0..X {
        for y in 0..Y {
            if input[x][y] == b'@' {
                let mut adjacent_count = 0;
                for (dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < X as isize && ny >= 0 && ny < Y as isize {
                        if input[nx as usize][ny as usize] == b'@' {
                            adjacent_count += 1;
                        }
                    }
                }
                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut input: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let X = input.len();
    let Y = input[0].len();
    let mut count = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    loop {
        let mut changes = false;
        for x in 0..X {
            for y in 0..Y {
                if input[x][y] == b'@' {
                    let mut adjacent_count = 0;
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < X as isize && ny >= 0 && ny < Y as isize {
                            if input[nx as usize][ny as usize] == b'@' {
                                adjacent_count += 1;
                            }
                        }
                    }
                    if adjacent_count < 4 {
                        count += 1;
                        changes = true;
                        input[x][y] = b'x';
                    }
                }
            }
        }
        if !changes {
            break;
        }
    }

    Ok(count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = solve_part1(input)?;
        assert_eq!(result.to_string(), "13");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = solve_part2(input)?;
        assert_eq!(result.to_string(), "43");

        Ok(())
    }
}
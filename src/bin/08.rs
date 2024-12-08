use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // Debugging
    fn print_state(input: &Vec<Vec<u8>>, antinodes: &Vec<Vec<bool>>) {
        let height = input.len();
        let width = input[0].len();

        for y in 0..height {
            for x in 0..width {
                if antinodes[x][y] {
                    print!("#");
                } else {
                    print!("{}", char::from(input[y][x]));
                }
            }
            println!();
        }
        println!();
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        let mut antinodes = vec![vec![false; height]; width];
        for y1 in 0..height as i32 {
            for x1 in 0..width as i32 {
                let frequency = input[y1 as usize][x1 as usize];
                if frequency == b'.' {
                    continue;
                }
                for y2 in 0..height as i32 {
                    for x2 in 0..width as i32 {
                        if y1 == y2 && x1 == x2 {
                            continue;
                        }

                        let other_frequency = input[y2 as usize][x2 as usize];
                        if other_frequency == frequency {
                            // println!("Pairing {} ({}, {}) with {} ({}, {})", char::from(frequency), x1, y1, char::from(other_frequency), x2, y2);
                            let dx = x2 - x1;
                            let dy = y2 - y1;

                            if x2 + dx >= 0 && x2 + dx < width as i32 && y2 + dy >= 0 && y2 + dy < height as i32 {
                                antinodes[(x2 + dx) as usize][(y2 + dy) as usize] = true;
                            }
                            if x1 - dx >= 0 && x1 - dx < width as i32 && y1 - dy >= 0 && y1 - dy < height as i32 {
                                antinodes[(x1 - dx) as usize][(y1 - dy) as usize] = true;
                            }
                            // print_state(&input, &antinodes);
                        }
                    }
                }
            }
        }
        // print_state(&input, &antinodes);

        let answer = antinodes.iter().flatten().filter(|&&b| b).count();

        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();
        
        let mut antinodes = vec![vec![false; height]; width];
        for y1 in 0..height as i32 {
            for x1 in 0..width as i32 {
                let frequency = input[y1 as usize][x1 as usize];
                if frequency == b'.' {
                    continue;
                }
                for y2 in 0..height as i32 {
                    for x2 in 0..width as i32 {
                        if y1 == y2 && x1 == x2 {
                            continue;
                        }

                        let other_frequency = input[y2 as usize][x2 as usize];
                        if other_frequency == frequency {
                            // println!("Pairing {} ({}, {}) with {} ({}, {})", char::from(frequency), x1, y1, char::from(other_frequency), x2, y2);
                            let dx = x2 - x1;
                            let dy = y2 - y1;

                            let mut x = x1;
                            let mut y = y1;
                            while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                                antinodes[x as usize][y as usize] = true;
                                x += dx;
                                y += dy;
                            }
                            while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                                antinodes[x as usize][y as usize] = true;
                                x -= dx;
                                y -= dy;
                            }
                            // print_state(&input, &antinodes);
                        }
                    }
                }
            }
        }
        // print_state(&input, &antinodes);

        let answer = antinodes.iter().flatten().filter(|&&b| b).count();

        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

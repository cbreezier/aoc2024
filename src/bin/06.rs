use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

// In order of "turning right"
// (0, 0) is the top left
const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        let mut pos: [i32; 2] = [0, 0];
        let mut dir_index = 3; // Up
        // Find the initial position
        for y in 0..height {
            for x in 0..width {
                if input[y][x] == b'^' {
                    pos = [x as i32, y as i32];
                }
            }
        }

        // Start walking the guard
        loop {
            // println!("At pos {}, {}", pos[0], pos[1]);
            // Mark current position as walked
            input[pos[1] as usize][pos[0] as usize] = b'X';

            // Move and mark new position as walked
            let dir = DIRS[dir_index];
            let next_pos: [i32; 2] = [
                pos[0] + dir[0],
                pos[1] + dir[1],
            ];

            // We're about to exit
            if next_pos[0] < 0 || next_pos[0] >= width as i32 || next_pos[1] < 0 || next_pos[1] >= height as i32 {
                break;
            }

            if input[next_pos[1] as usize][next_pos[0] as usize] == b'#' {
                // Turn right
                // println!("Turn right");
                dir_index = (dir_index + 1) % DIRS.len();
            } else {
                pos = next_pos;
            }
        }
        // println!("Exited at {} {}", pos[0], pos[1]);

        // Count walked tiles
        let mut num_walked = 0;
        for y in 0..height {
            for x in 0..width {
                if input[y][x] == b'X' {
                    num_walked += 1;
                }
            }
        }

        Ok(num_walked)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn does_loop(input: &Vec<Vec<u8>>, initial_pos: [i32; 2]) -> bool {
        let height = input.len();
        let width = input[0].len();

        // Stores whether we've been at this [y][x][dir_index]
        let mut seen = vec![vec![[false; 4]; width]; height];

        let mut pos: [i32; 2] = initial_pos;
        let mut dir_index = 3; // Up

        // Start walking the guard
        loop {
            // println!("At pos {}, {} facing {}", pos[0], pos[1], dir_index);

            if seen[pos[1] as usize][pos[0] as usize][dir_index] {
                // We've already been here, facing this direction...it's a loop!
                return true;
            }
            // Mark current position as walked
            seen[pos[1] as usize][pos[0] as usize][dir_index] = true;

            // Try move
            let dir = DIRS[dir_index];
            let next_pos: [i32; 2] = [
                pos[0] + dir[0],
                pos[1] + dir[1],
            ];

            // We're about to exit
            if next_pos[0] < 0 || next_pos[0] >= width as i32 || next_pos[1] < 0 || next_pos[1] >= height as i32 {
                return false;
            }

            if input[next_pos[1] as usize][next_pos[0] as usize] == b'#' {
                // Turn right
                // println!("Turn right");
                dir_index = (dir_index + 1) % DIRS.len();
            } else {
                pos = next_pos;
            }
        }
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        // Find the initial position, replacing it with an X
        let mut initial_pos: [i32; 2] = [0, 0];
        for y in 0..height {
            for x in 0..width {
                if input[y][x] == b'^' {
                    initial_pos = [x as i32, y as i32];
                }
            }
        }

        // Brute-force: try placing an obstacle at every possible spot and check if it loops
        let mut num_loops = 0;
        for y in 0..height {
            for x in 0..width {
                if input[y][x] != b'#' && input[y][x] != b'^' {
                    // println!("Trying an obstacle at {} {}", x, y);
                    // Try turning it into an obstacle
                    input[y][x] = b'#';
                    if does_loop(&input, initial_pos) {
                        // println!("{} {} loops!", x, y);
                        num_loops += 1;
                    }
                    // Remove the obstacle for the next test
                    input[y][x] = b'.';
                }
            }
        }

        Ok(num_loops)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

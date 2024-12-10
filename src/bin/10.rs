use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

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

    fn dfs(input: &Vec<Vec<u8>>, x: i32, y: i32) -> usize {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        stack.push((x, y));

        let height = input.len();
        let width = input[0].len();

        let mut seen = vec![vec![false; width]; height];

        let mut answer = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            let cur_height = input[y as usize][x as usize];
            // println!("At x: {}, y: {}, height: {}", x, y, char::from(cur_height));

            if cur_height == b'9' {
                if seen[x as usize][y as usize] == false {
                    seen[x as usize][y as usize] = true;
                    answer += 1;
                }
                continue;
            }

            for dir in DIRS {
                let new_x = x + dir[0];
                let new_y = y + dir[1];
                if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                    continue;
                }

                let new_height = input[new_y as usize][new_x as usize];

                if new_height == cur_height + 1 {
                    // println!("Pushing {} at {}, {}", char::from(new_height), new_x, new_y);
                    stack.push((new_x, new_y));
                }
            }
        }

        answer
    }
    
    fn dfs2(input: &Vec<Vec<u8>>, x: i32, y: i32) -> usize {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        stack.push((x, y));

        let height = input.len();
        let width = input[0].len();

        let mut answer = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            let cur_height = input[y as usize][x as usize];
            // println!("At x: {}, y: {}, height: {}", x, y, char::from(cur_height));

            if cur_height == b'9' {
                answer += 1;
                continue;
            }

            for dir in DIRS {
                let new_x = x + dir[0];
                let new_y = y + dir[1];
                if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                    continue;
                }

                let new_height = input[new_y as usize][new_x as usize];

                if new_height == cur_height + 1 {
                    // println!("Pushing {} at {}, {}", char::from(new_height), new_x, new_y);
                    stack.push((new_x, new_y));
                }
            }
        }

        answer
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        let mut answer = 0usize;
        for x in 0..width {
            for y in 0..height {
                if input[y][x] == b'0' {
                    let num_trails = dfs(&input, x as i32, y as i32);
                    // println!("Found {} trails starting from {}, {}", num_trails, x, y);
                    answer += num_trails;
                }
            }
        }

        // println!("num trails: {}", dfs(&input, 6, 6));

        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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

        let mut answer = 0usize;
        for x in 0..width {
            for y in 0..height {
                if input[y][x] == b'0' {
                    let num_trails = dfs2(&input, x as i32, y as i32);
                    // println!("Found {} trails starting from {}, {}", num_trails, x, y);
                    answer += num_trails;
                }
            }
        }

        // println!("num trails: {}", dfs(&input, 6, 6));

        Ok(answer)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

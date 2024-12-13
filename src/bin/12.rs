use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
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

    fn dfs(input: &Vec<Vec<u8>>, x: i32, y: i32, seen: &mut HashSet<(i32, i32)>) -> usize {
        let height = input.len();
        let width = input[0].len();
        
        let mut stack: Vec<(i32, i32)> = vec![(x, y)];
        let mut perimeter = 0usize;
        let mut area = 0usize;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));
            
            area += 1;

            let cur_val = input[y as usize][x as usize];
            for dir in DIRS.iter() {
                let next_x = x + dir[0];
                let next_y = y + dir[1];
                
                if next_x < 0 || next_x >= width as i32 || next_y < 0 || next_y >= height as i32 {
                    perimeter += 1;
                    continue;
                }
                
                let next_val = input[next_y as usize][next_x as usize];

                if next_val != cur_val {
                    perimeter += 1;
                } else {
                    stack.push((next_x, next_y));
                }
            }
        }
        
        area * perimeter
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut answer = 0usize;
        for x in 0..(width as i32) {
            for y in 0..(height as i32) {
                answer += dfs(&input, x, y, &mut seen);
            }
        }

        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn dfs2(input: &Vec<Vec<u8>>, x: i32, y: i32, seen: &mut HashSet<(i32, i32)>) -> usize {
        let height = input.len();
        let width = input[0].len();

        let mut stack: Vec<(i32, i32)> = vec![(x, y)];
        let mut perimeter = 0usize;
        let mut area = 0usize;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));

            area += 1;

            let cur_val = input[y as usize][x as usize];
            for dir_index in 0..DIRS.len() {
                let dir = DIRS[dir_index];
                let next_x = x + dir[0];
                let next_y = y + dir[1];
                
                let next_dir = DIRS[(dir_index + 1) % DIRS.len()];
                let next_next_x = x + next_dir[0];
                let next_next_y = y + next_dir[1];
                
                let diag_x = x + dir[0] + next_dir[0];
                let diag_y = y + dir[1] + next_dir[1];

                let side1 = (next_x < 0 || next_x >= width as i32 || next_y < 0 || next_y >= height as i32) ||
                    input[next_y as usize][next_x as usize] != cur_val;
                
                let side2 = (next_next_x < 0 || next_next_x >= width as i32 || next_next_y < 0 || next_next_y >= height as i32) ||
                    input[next_next_y as usize][next_next_x as usize] != cur_val;
                
                let diag = (diag_x < 0 || diag_x >= width as i32 || diag_y < 0 || diag_y >= height as i32) ||
                    input[diag_y as usize][diag_x as usize] != cur_val;

                if side1 && side2 {
                    // Outer corner
                    perimeter += 1;
                } else if !side1 && !side2 && diag {
                    // Inner corner
                    perimeter += 1;
                }
                
                if !side1 {
                    stack.push((next_x, next_y));
                }
            }
        }

        area * perimeter
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = input.len();
        let width = input[0].len();

        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut answer = 0usize;
        for x in 0..(width as i32) {
            for y in 0..(height as i32) {
                answer += dfs2(&input, x, y, &mut seen);
            }
        }

        Ok(answer)
    }
    
    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

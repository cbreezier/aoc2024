use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const DIRS: [[i32; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let width = input[0].len();
        let height = input.len();

        let mut answer = 0;
        // For each position in the matrix
        for i in 0..height {
            for j in 0..width {
                // For each direction
                for dir in DIRS {
                    // For each character in XMAS
                    let mut found = true;
                    for c in 0..4 {
                        let y = i as i32 + dir[0] * c;
                        let x = j as i32 + dir[1] * c;
                        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                            found = false;
                            break;
                        }
                        let char = input[y as usize].as_bytes()[x as usize];
                        if char != "XMAS".as_bytes()[c as usize] {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        answer += 1;
                    }
                }
            }
        }
        
        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let width = input[0].len();
        let height = input.len();

        let mut answer = 0;
        // For each position in the matrix
        for i in 1..height - 1 {
            for j in 1..width - 1 {
                let c = input[i].as_bytes()[j];
                // Top left, top right, bottom right, bottom left
                let c_tl = input[i - 1].as_bytes()[j - 1];
                let c_tr = input[i - 1].as_bytes()[j + 1];
                let c_br = input[i + 1].as_bytes()[j + 1];
                let c_bl = input[i + 1].as_bytes()[j - 1];
                
                if c == b'A' && ((c_tl == b'M' && c_br == b'S') || (c_tl == b'S' && c_br == b'M')) && ((c_tr == b'M' && c_bl == b'S') || (c_tr == b'S' && c_bl == b'M')) {
                    answer += 1;
                }
            }
        }
        Ok(answer)
    }
    
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

use std::collections::{HashSet, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

// (0, 0) is the top left
const DIRS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn bfs(corruptions: &HashSet<(i32, i32)>, size: usize) -> Option<usize> {
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<((i32, i32), usize)> = VecDeque::new();
        queue.push_front(((0, 0), 0));
        while !queue.is_empty() {
            let ((x, y), cost) = queue.pop_front().unwrap();

            if x == size as i32 - 1 && y == size as i32 - 1 {
                return Some(cost);
            }
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));

            for dir in DIRS {
                let new_x = x + dir.0;
                let new_y = y + dir.1;
                if new_x < 0 || new_x >= size as i32 || new_y < 0 || new_y >= size as i32 {
                    continue;
                }
                if corruptions.contains(&(new_x, new_y)) {
                    continue;
                }
                queue.push_back(((new_x, new_y), cost + 1));
            }
        }

        None
    }

    fn part1<R: BufRead>(reader: R, size: usize, num_fallen: usize) -> Result<usize> {
        let input = reader.lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect::<Vec<(i32, i32)>>();

        let mut corruptions = HashSet::<(i32, i32)>::new();
        for i in 0..num_fallen {
            corruptions.insert(input[i]);
        }

        // for y in 0..size {
        //     for x in 0..size {
        //         if corruptions.contains(&(x as i32, y as i32)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        let answer = bfs(&corruptions, size);
        answer.ok_or(Error::msg("No solution found"))
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, size: usize) -> Result<(i32, i32)> {
        let input = reader.lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect::<Vec<(i32, i32)>>();

        let mut corruptions = HashSet::<(i32, i32)>::new();
        for i in 0..input.len() {
            corruptions.insert(input[i]);

            let answer = bfs(&corruptions, size);
            match answer {
                Some(_) => continue,
                None => return Ok(input[i]),
            }
        }

        Err(Error::msg("No solution found"))
    }

    assert_eq!((6, 1), part2(BufReader::new(TEST.as_bytes()), 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71)?);
    println!("Result = {},{}", result.0, result.1);
    //endregion

    Ok(())
}

use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use scan_fmt::scan_fmt_some;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        let mut quadrants = [0usize; 4];
        reader.lines()
            .map(|l| l.unwrap())
            .for_each(|l| {
                let (x, y, vx, vy) = scan_fmt_some!(&l, "p={d},{d} v={d},{d}", i32, i32, i32, i32);
                // println!("Solving for ({}, {}) at velocity {}, {}", x.unwrap(), y.unwrap(), vx.unwrap(), vy.unwrap());

                let end_pos = (
                    (((x.unwrap() + vx.unwrap() * 100) % width) + width) % width,
                    (((y.unwrap() + vy.unwrap() * 100) % height) + height) % height,
                );
                // println!("  Ending at {}, {}", end_pos.0, end_pos.1);

                if end_pos.0 < width / 2 {
                    if end_pos.1 < height / 2 {
                        quadrants[0] += 1;
                    }
                    if end_pos.1 > height / 2 {
                        quadrants[3] += 1;
                    }
                }
                if end_pos.0 > width / 2 {
                    if end_pos.1 < height / 2 {
                        quadrants[1] += 1;
                    }
                    if end_pos.1 > height / 2 {
                        quadrants[2] += 1;
                    }
                }
            });

        // println!("Quadrants {}, {}, {}, {}", quadrants[0], quadrants[1], quadrants[2], quadrants[3]);
        Ok(quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3])
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    struct Precondition {
        x: i32,
        y: i32,
        vx: i32,
        vy: i32,
    }

    fn part2<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        let input: Vec<Precondition> = reader.lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let (x, y, vx, vy) = scan_fmt_some!(&l, "p={d},{d} v={d},{d}", i32, i32, i32, i32);
                Precondition {
                    x: x.unwrap(),
                    y: y.unwrap(),
                    vx: vx.unwrap(),
                    vy: vy.unwrap(),
                }
            })
            .collect();

        for time in 0..100_000 {
            let mut count = [[0; 101]; 103];
            let mut avg_x = 0usize;
            let mut avg_y = 0usize;

            // Compute ending positions
            input.iter().for_each(|Precondition { x, y, vx, vy }| {
                let end_pos = (
                    (((x + vx * time) % width) + width) % width,
                    (((y + vy * time) % height) + height) % height,
                );

                count[end_pos.1 as usize][end_pos.0 as usize] += 1;

                avg_x += end_pos.0 as usize;
                avg_y += end_pos.1 as usize;
            });

            // Try and find something that seems less random
            avg_x = avg_x / input.len();
            avg_y = avg_y / input.len();
            let found_solution = avg_x < 40 || avg_x > 60 || avg_y < 40 || avg_y > 60;

            // if quadrants[0] != quadrants[1] || quadrants[2] != quadrants[3] {
            //     // Cheap check before we check properly
            //     continue;
            // }

            // A Christmas tree must be horizontally symmetric
            // let mut found_solution = true;
            // for x in 0..width {
            //     for y in 0..height {
            //         let count_here = count[y as usize][x as usize];
            //         let count_there = count[y as usize][(width - 1 - x) as usize];
            //         if (count_here > 0 && count_there == 0) || (count_here == 0 && count_there > 0) {
            //             found_solution = false;
            //         }
            //     }
            // }

            if found_solution {
                for y in 0..height {
                    for x in 0..width {
                        if count[y as usize][x as usize] == 0 {
                            print!(".");
                        } else {
                            print!("{}", count[y as usize][x as usize]);
                        }
                    }
                    println!();
                }
                println!("Found solution at time {}", time);
                thread::sleep(Duration::from_millis(1000));
            }
        }

        Ok(0)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    time_snippet!(part2(input_file, 101, 103)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

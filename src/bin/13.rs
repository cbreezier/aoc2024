use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use scan_fmt::scan_fmt_some;
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: (usize, usize),
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // Min-heap
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn dijkstra(goal: (usize, usize), a: (usize, usize), b: (usize, usize)) -> Option<usize> {
        let mut cache: HashSet<(usize, usize)> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: (0, 0)});
        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                return Some(cost);
            }
            if position.0 > goal.0 || position.1 > goal.1 {
                // We've overshot - no point continuing this idea
                continue;
            }

            if cache.contains(&position) {
                continue;
            }
            cache.insert(position);

            heap.push(State { cost: cost + 3, position: (position.0 + a.0, position.1 + a.1)});
            heap.push(State { cost: cost + 1, position: (position.0 + b.0, position.1 + b.1)});
        }

        None
    }

    // There is only one possible solution, the "find the minimum tokens" is a red herring
    // Imagine it as two vectors that add to a goal vector. We binary search the length of the
    // first vector such that the second vector can reach the goal with an integer multiple.
    fn f1(goal: (usize, usize), a: (usize, usize), b: (usize, usize)) -> Option<usize> {
        let low_slope = (goal.0 as f64 / a.0 as f64) - (goal.1 as f64 / a.1 as f64) < 0.0;
        let mut num_b_low = 0;
        let mut num_b_high = min(goal.0 / b.0, goal.1 / b.1);
        while num_b_high > num_b_low {
            let num_b = (num_b_low + num_b_high) / 2;
            // println!("{} ({} to {})", num_b, num_b_low, num_b_high);
            let remaining_x = goal.0 - (b.0 * num_b);
            let remaining_y = goal.1 - (b.1 * num_b);

            let num_a_x = remaining_x / a.0;
            let num_a_y = remaining_y / a.1;

            if remaining_x % a.0 == 0 && remaining_y % a.1 == 0 && num_a_x == num_a_y {
                // We've found the solution
                // println!("    Found solution {} a's and {} b's for total {}", num_a_x, num_b, num_a_x * 3 + num_b);
                return Some(num_a_x * 3 + num_b);
            }

            let slope = (remaining_x as f64 / a.0 as f64) - (remaining_y as f64 / a.1 as f64) < 0.0;
            if slope == low_slope {
                // println!("  Low side of slope: {} {}", num_a_x, num_a_y);
                // We're still on the low side of num_b
                num_b_low = num_b + 1;
            } else {
                // println!("  High side of slope: {} {}", num_a_x, num_a_y);
                num_b_high = num_b;
            }
        }

        None
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let mut answer = 0usize;
        all_input.split("\n\n").for_each(|block| {
            let lines = block.split("\n").collect::<Vec<&str>>();
            let (a_x, a_y) = scan_fmt_some!(lines[0], "Button A: X+{d}, Y+{d}", usize, usize);
            let (b_x, b_y) = scan_fmt_some!(lines[1], "Button B: X+{d}, Y+{d}", usize, usize);
            let (p_x, p_y) = scan_fmt_some!(lines[2], "Prize: X={d}, Y={d}", usize, usize);

            // println!("Trying to reach {}, {} using A {}, {} and B {}, {}", p_x.unwrap(), p_y.unwrap(), a_x.unwrap(), a_y.unwrap(), b_x.unwrap(), b_y.unwrap());
            let min_cost_old = dijkstra((p_x.unwrap(), p_y.unwrap()), (a_x.unwrap(), a_y.unwrap()), (b_x.unwrap(), b_y.unwrap())).unwrap_or_else(|| 0usize);
            let min_cost = f1((p_x.unwrap(), p_y.unwrap()), (a_x.unwrap(), a_y.unwrap()), (b_x.unwrap(), b_y.unwrap())).unwrap_or_else(|| 0usize);
            if min_cost_old != min_cost {
                println!("Trying to reach {}, {} using A {}, {} and B {}, {} (differ {} to {})", p_x.unwrap(), p_y.unwrap(), a_x.unwrap(), a_y.unwrap(), b_x.unwrap(), b_y.unwrap(), min_cost_old, min_cost);
            }
            answer += min_cost;
        });
        Ok(answer)
    }

    // f1((1409, 2686), (96, 72), (17, 46));

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let mut answer = 0usize;
        all_input.split("\n\n").for_each(|block| {
            let lines = block.split("\n").collect::<Vec<&str>>();
            let (a_x, a_y) = scan_fmt_some!(lines[0], "Button A: X+{d}, Y+{d}", usize, usize);
            let (b_x, b_y) = scan_fmt_some!(lines[1], "Button B: X+{d}, Y+{d}", usize, usize);
            let (p_x, p_y) = scan_fmt_some!(lines[2], "Prize: X={d}, Y={d}", usize, usize);

            // println!("Trying to reach {}, {} using A {}, {} and B {}, {}", p_x.unwrap(), p_y.unwrap(), a_x.unwrap(), a_y.unwrap(), b_x.unwrap(), b_y.unwrap());
            let min_cost = f1((p_x.unwrap() + 10000000000000, p_y.unwrap() + 10000000000000), (a_x.unwrap(), a_y.unwrap()), (b_x.unwrap(), b_y.unwrap())).unwrap_or_else(|| 0usize);
            // println!("  min cost {}", min_cost);
            answer += min_cost;
        });
        Ok(answer)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

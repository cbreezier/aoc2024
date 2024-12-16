use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
const TEST2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
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

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: (i32, i32),
        dir_index: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // Min-heap
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
                .then_with(|| self.dir_index.cmp(&other.dir_index))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn dijkstra(map: &Vec<Vec<u8>>, goal: (i32, i32), initial_pos: (i32, i32)) -> Option<usize> {
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut cache: HashSet<((i32, i32), usize)> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: initial_pos, dir_index: 0 });
        while let Some(State { cost, position, dir_index }) = heap.pop() {
            if position == goal {
                return Some(cost);
            }

            if cache.contains(&(position, dir_index)) {
                continue;
            }
            cache.insert((position, dir_index));

            // Go straight
            let dir = DIRS[dir_index];
            let new_pos = (position.0 + dir.0, position.1 + dir.1);
            if new_pos.0 >= 0 && new_pos.0 < width && new_pos.1 >= 0 && new_pos.1 < height {
                if map[new_pos.1 as usize][new_pos.0 as usize] != b'#' {
                    heap.push(State { cost: cost + 1, position: new_pos, dir_index });
                }
            }

            // Turn left or right
            heap.push(State { cost: cost + 1000, position, dir_index: (dir_index + 1) % 4 });
            heap.push(State { cost: cost + 1000, position, dir_index: (dir_index + 3) % 4 });
        }

        None
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = map.len();
        let width = map[0].len();

        let mut initial_pos = (0, 0);
        let mut goal = (0, 0);
        for x in 0..width {
            for y in 0..height {
                if map[y][x] == b'S' {
                    initial_pos = (x as i32, y as i32);
                }
                if map[y][x] == b'E' {
                    goal = (x as i32, y as i32);
                }
            }
        }

        dijkstra(&map, goal, initial_pos).ok_or(Error::msg("No path found"))
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct StateWithPrev {
        cost: usize,
        position: (i32, i32),
        dir_index: usize,
        prev: Option<((i32, i32), usize)>,
    }

    impl Ord for StateWithPrev {
        fn cmp(&self, other: &Self) -> Ordering {
            // Min-heap
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
                .then_with(|| self.dir_index.cmp(&other.dir_index))
                .then_with(|| self.prev.cmp(&other.prev))
        }
    }
    impl PartialOrd for StateWithPrev {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // With storing the path we got here from
    fn dijkstra2(map: &Vec<Vec<u8>>, goal: (i32, i32), initial_pos: (i32, i32)) -> Option<usize> {
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        // All the ways that we've arrived at the key
        let mut prevs: HashMap<((i32, i32), usize), Vec<((i32, i32), usize)>> = HashMap::new();

        // Map of (pos, dir) to the lowest cost we reached here
        let mut cache: HashMap<((i32, i32), usize), usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        heap.push(StateWithPrev { cost: 0, position: initial_pos, dir_index: 0, prev: None });
        while let Some(StateWithPrev { cost, position, dir_index, prev }) = heap.pop() {
            let visited = cache.get(&(position, dir_index));
            match visited {
                Some(last_cost) => {
                    if cost > *last_cost {
                        if position == goal {
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        // Record another way in which we got here in an optimal way
                        match prev {
                            Some(prev_pos) => {
                                prevs.entry((position, dir_index)).or_default().push(prev_pos);
                            },
                            None => {}
                        }
                    }
                },
                None => {
                    cache.insert((position, dir_index), cost);
                    // Record another way in which we got here in an optimal way
                    match prev {
                        Some(prev_pos) => {
                            prevs.entry((position, dir_index)).or_default().push(prev_pos);
                        },
                        None => {}
                    }
                }
            }

            // Go straight
            let dir = DIRS[dir_index];
            let new_pos = (position.0 + dir.0, position.1 + dir.1);
            if new_pos.0 >= 0 && new_pos.0 < width && new_pos.1 >= 0 && new_pos.1 < height {
                if map[new_pos.1 as usize][new_pos.0 as usize] != b'#' {
                    heap.push(StateWithPrev { cost: cost + 1, position: new_pos, dir_index , prev: Some((position, dir_index))});
                }
            }

            // Turn left or right
            heap.push(StateWithPrev { cost: cost + 1000, position, dir_index: (dir_index + 1) % 4, prev });
            heap.push(StateWithPrev { cost: cost + 1000, position, dir_index: (dir_index + 3) % 4, prev });
        }

        // Time to walk the prevs to find all paths that made it to the goal
        println!("Walking prevs");
        // if width > 15 {
        //     println!("Prevs of 15, 1");
        //     for p in prevs.get(&(15, 1)).unwrap() {
        //         println!("  {}, {}", p.0, p.1);
        //     }
        // }
        let mut back_stack: Vec<((i32, i32), usize)> = vec![(goal, 0), (goal, 1), (goal, 2), (goal, 3)];
        let mut back_seen: HashSet<(i32, i32)> = HashSet::new();
        while !back_stack.is_empty() {
            let back_pos = back_stack.pop().unwrap();
            if back_seen.contains(&back_pos.0) {
                continue;
            }
            back_seen.insert(back_pos.0);

            match prevs.get(&back_pos) {
                Some(prev_poses) => {
                    for prev_pos in prev_poses {
                        back_stack.push(*prev_pos);
                    }
                },
                None => continue
            }
        }

        for y in 0..height {
            for x in 0..width {
                if back_seen.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!("{}", char::from(map[y as usize][x as usize]));
                }
            }
            println!();
        }

        Some(back_seen.len())
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader.lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();

        let height = map.len();
        let width = map[0].len();

        let mut initial_pos = (0, 0);
        let mut goal = (0, 0);
        for x in 0..width {
            for y in 0..height {
                if map[y][x] == b'S' {
                    initial_pos = (x as i32, y as i32);
                }
                if map[y][x] == b'E' {
                    goal = (x as i32, y as i32);
                }
            }
        }

        dijkstra2(&map, goal, initial_pos).ok_or(Error::msg("No path found"))
    }

    println!("Test 1");
    assert_eq!(45, part2(BufReader::new(TEST.as_bytes()))?);
    println!("Test 2");
    assert_eq!(64, part2(BufReader::new(TEST2.as_bytes()))?);

    println!("File input");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

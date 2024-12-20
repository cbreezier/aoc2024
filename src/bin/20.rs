use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
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
    #[derive(Hash)]
    struct State {
        cost: usize,
        position: (i32, i32),
        cheat_location: Option<(i32, i32)>,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // Min-heap
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
                .then_with(|| self.cheat_location.cmp(&other.cheat_location))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn dijkstra_shortest(map: &Vec<Vec<u8>>, cache: &mut HashMap<(i32, i32), usize>, goal: (i32, i32), initial_pos: (i32, i32)) -> Option<usize> {
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: goal, cheat_location: None });
        let mut shortest_path: Option<usize> = None;
        while let Some(State { cost, position, cheat_location }) = heap.pop() {
            if position == initial_pos {
                shortest_path = Some(cost);
            }

            if cache.contains_key(&position) {
                continue;
            }
            cache.insert(position, cost);

            for dir in DIRS {
                let new_pos = (position.0 + dir.0, position.1 + dir.1);
                if new_pos.0 >= 0 && new_pos.0 < width && new_pos.1 >= 0 && new_pos.1 < height {
                    if map[new_pos.1 as usize][new_pos.0 as usize] != b'#' {
                        heap.push(State { cost: cost + 1, position: new_pos, cheat_location });
                    }
                }
            }
        }

        shortest_path
    }

    // cache tracks distance from position to goal
    fn dijkstra_count_shortest(map: &Vec<Vec<u8>>, cache: &HashMap<(i32, i32), usize>, goal: (i32, i32), initial_pos: (i32, i32), cost_to_beat: usize) -> usize {
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut seen: HashSet<((i32, i32), Option<(i32, i32)>)> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(State { cost: 0, position: initial_pos, cheat_location: None });

        // Cheat location to cost
        let mut answers: HashMap<usize, usize> = HashMap::new();
        // let mut answer = 0usize;
        while let Some(state) = heap.pop() {
            let State { cost, position, cheat_location } = state;
            if cost > cost_to_beat {
                // println!("Cost is already {} at {}, {}, exiting", cost, position.0, position.1);
                break;
            }

            if position == goal && cheat_location == None {
                break;
            }

            if seen.contains(&(position, cheat_location)) {
                continue;
            }
            seen.insert((position, cheat_location));

            // println!("At {}, {} with cost {} and {} cheats left", position.0, position.1, cost, cheats_left);
            match cheat_location {
                Some(cheat_location) => {
                    if position != cheat_location {
                        if map[position.1 as usize][position.0 as usize] == b'#' {
                            continue;
                        }

                        let distance_to_goal = cache.get(&position);
                        match distance_to_goal {
                            Some(d) => {
                                if cost + *d <= cost_to_beat {
                                    // println!("Found answer with cost {} at {}, {} by cheating at {}, {}", cost + *d, position.0, position.1, cheat_location.0, cheat_location.1);
                                    *answers.entry(cost + *d).or_default() += 1;
                                }
                                continue;
                            }
                            None => {
                                panic!("Surprising - we don't know how long to the goal")
                            }
                        }
                    } else {
                        for dir in DIRS {
                            let new_pos = (position.0 + dir.0, position.1 + dir.1);
                            if new_pos.0 >= 0 && new_pos.0 < width && new_pos.1 >= 0 && new_pos.1 < height {
                                if map[new_pos.1 as usize][new_pos.0 as usize] != b'#' {
                                    heap.push(State { cost: cost + 1, position: new_pos, cheat_location: Some(cheat_location) });
                                }
                            }
                        }
                    }
                }
                None => {
                    for dir in DIRS {
                        let new_pos = (position.0 + dir.0, position.1 + dir.1);
                        if new_pos.0 >= 0 && new_pos.0 < width && new_pos.1 >= 0 && new_pos.1 < height {
                            if map[new_pos.1 as usize][new_pos.0 as usize] != b'#' {
                                heap.push(State { cost: cost + 1, position: new_pos, cheat_location: None });
                            } else {
                                heap.push(State { cost: cost + 1, position: new_pos, cheat_location: Some(new_pos) });
                            }
                        }
                    }
                }
            }
        }

        // for answer in answers.iter() {
        //     println!("There are {} cheats that cost {}", answer.1, answer.0);
        // }
        answers.values().sum()
    }

    fn part1<R: BufRead>(reader: R, save_at_least: usize) -> Result<usize> {
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

        let mut cache: HashMap<(i32, i32), usize> = HashMap::new();
        let shortest_path = dijkstra_shortest(&map, &mut cache, goal, initial_pos).unwrap();
        // println!("Shortest path: {}", shortest_path);
        let answer = dijkstra_count_shortest(&map, &cache, goal, initial_pos, shortest_path - save_at_least);
        Ok(answer)
    }

    assert_eq!(44, part1(BufReader::new(TEST.as_bytes()), 1)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 100)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn num_ways(question: &str, towels: &Vec<String>, index: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if cache.contains_key(&index) {
            return cache[&index];
        }
        if index >= question.len() {
            return 1;
        }
        let mut answer = 0usize;
        for towel in towels {
            let cur_q = &question[index..];
            if cur_q.starts_with(towel) {
                answer += num_ways(question, towels, index + towel.len(), cache);
            }
        }
        cache.insert(index, answer);
        answer
    }
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (towels_str, questions_str) = all_input.split_once("\n\n").unwrap();
        let towels = towels_str.split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let questions = questions_str.trim().split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let answer = questions.iter()
            .map(|q| {
                let mut cache = HashMap::new();
                let result = num_ways(&q, &towels, 0, &mut cache);
                result
            })
            .sum();
        Ok(answer)
    }
    
    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

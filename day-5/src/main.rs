#![allow(unused)]

use std::collections::{HashMap, VecDeque};

struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        parts.next(); // skip the `move` keyword
        let count: usize = parts.next().unwrap().parse().unwrap();
        parts.next(); // skip the `from` keyword
        let from: usize = parts.next().unwrap().parse().unwrap();
        parts.next(); // skip the `to` keyword
        let to: usize = parts.next().unwrap().parse().unwrap();

        Movement { count, from, to }
    }
}

struct Cargo {
    stacks: HashMap<usize, VecDeque<char>>,
    movements: Vec<Movement>,
}

impl Cargo {
    fn new(stacks: HashMap<usize, VecDeque<char>>, movements: Vec<Movement>) -> Self {
        Self { stacks, movements }
    }

    fn rearrange(&mut self) {
        for movement in &self.movements {
            let from = self.stacks.get_mut(&movement.from).unwrap();
            let to_move = {
                if !from.is_empty() && movement.count <= from.len() {
                    from.drain((from.len() - movement.count)..).collect()
                } else {
                    VecDeque::<char>::new()
                }
            };

            let to = self.stacks.get_mut(&movement.to).unwrap();
            to.extend(to_move);
        }
    }

    fn get_top_crates(&mut self) -> String {
        let mut result = String::with_capacity(self.stacks.len());
        for i in 0..self.stacks.len() {
            if let Some(stack) = self.stacks.get(&(i + 1)) {
                result.push(stack.back().unwrap().to_owned());
            }
        }
        result
    }
}

fn main() {
    let input = include_str!("input.txt").replace("\n\r", "\n\n");
    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();
    let stack_number = (&stacks_str.lines().last().unwrap().len() + 1) / 4;
    let stacks = parse_stacks(stacks_str);

    let movements = instructions_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(Movement::from)
        .collect::<Vec<_>>();

    let mut cargo = Cargo::new(stacks, movements);
    cargo.rearrange();
    println!("Result = {:?}", cargo.get_top_crates());
}

fn parse_stacks(stacks_str: &str) -> HashMap<usize, VecDeque<char>> {
    let stack_number = (&stacks_str.lines().last().unwrap().len() + 1) / 4;

    let mut stacks = HashMap::<usize, VecDeque<char>>::with_capacity(stack_number);
    let mut lines = stacks_str.lines();
    for _ in 0..stack_number {
        let line = lines.next().unwrap();
        let chars = line.chars().collect::<Vec<_>>();

        for (j, ch) in chars.chunks(4).enumerate() {
            let s = stacks.entry(j + 1).or_insert(VecDeque::new());
            if ch[0] != '[' {
                continue;
            }

            s.push_front(ch[1]);
        }
    }

    stacks
}

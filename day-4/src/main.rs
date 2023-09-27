#![allow(unused)]

struct Range {
    low: usize,
    upp: usize,
}

impl Range {
    fn overlap(&self, other: &Range) -> bool {
        // if other low is inside my range
        (other.low >= self.low && other.low <= self.upp)
            // if other upp is inside my range
            || (other.upp >= self.low && other.upp <= self.upp)
    }
}

struct Pair {
    first: Range,
    second: Range,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (low, upp) = value.split_once('-').unwrap();

        Range {
            low: low.parse().unwrap(),
            upp: upp.parse().unwrap(),
        }
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once(',').unwrap();

        Pair {
            first: Range::from(first),
            second: Range::from(second),
        }
    }
}

fn main() {
    let mut result = 0;

    include_str!("input.txt")
        .lines()
        .map(Pair::from)
        .for_each(|pair| {
            if pair.first.overlap(&pair.second) || pair.second.overlap(&pair.first) {
                result += 1;
            }
        });

    println!("Result = {}", result);
}

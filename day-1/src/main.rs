fn main() {
    let input = include_str!("input.txt");

    let mut result = input
        .replace("\n\r", "\n\n")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|line| line.parse::<usize>().ok())
                .sum()
        })
        .collect::<Vec<usize>>();

    result.sort();
    let total = result.pop().unwrap() + result.pop().unwrap() + result.pop().unwrap();
    println!("Result = {:?}", total);
}

struct Elf {
    items: Vec<usize>,
}

fn main() {
    let input = include_str!("input.txt");

    let mut elves = Vec::<Elf>::new();
    let mut temp_items = Vec::<usize>::new();

    for line in input.lines() {
        if !line.is_empty() {
            let item = line.parse::<usize>().unwrap();
            temp_items.push(item);
            continue;
        }

        elves.push(Elf {
            items: temp_items.to_owned(),
        });

        temp_items.clear();
    }

    let result: usize = elves
        .iter()
        .map(|elf| elf.items.iter().sum())
        .max()
        .unwrap();

    println!("Result = {}", result);
}

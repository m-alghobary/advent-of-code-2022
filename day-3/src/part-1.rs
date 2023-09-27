fn main() {
    let input = include_str!("input.txt");

    let mut result = 0u32;
    for line in input.lines() {
        let (first_part, second_part) = line.split_at(line.len() / 2);

        for i in 0..first_part.len() {
            let ch = first_part.as_bytes()[i];
            if second_part.as_bytes().contains(&ch) {
                let priority: u32 = match ch {
                    b'a'..=b'z' => (ch - 96) as u32,
                    b'A'..=b'Z' => ((ch - 64) + 26) as u32,
                    _ => panic!("Unsupported char!"),
                };

                result += priority;
                print!("Duplicate {} - {} - {}", ch, ch as char, priority);
                break;
            }
        }

        println!();
    }
    println!("Result = {}", result);
}

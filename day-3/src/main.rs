fn main() {
    let input = include_str!("input.txt");

    let mut result = 0u32;
    for group in input.lines().collect::<Vec<_>>().chunks(3) {
        let first_line = group[0];
        let second_line = group[1];
        let thired_line = group[2];

        for i in 0..first_line.len() {
            let ch = first_line.as_bytes()[i];

            if second_line.as_bytes().contains(&ch) && thired_line.as_bytes().contains(&ch) {
                let priority: u32 = match ch {
                    b'a'..=b'z' => (ch - 96) as u32,
                    b'A'..=b'Z' => ((ch - 64) + 26) as u32,
                    _ => panic!("Unsupported char!"),
                };

                result += priority;
                print!("Badge {} - {} - {}", ch, ch as char, priority);
                break;
            }
        }

        println!();
    }
    println!("Result = {}", result);
}

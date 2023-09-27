fn main() {
    let input = include_bytes!("input.txt");

    let answer = input
        .windows(14)
        .position(|w| {
            let mut hash = Vec::with_capacity(14);
            for ch in w {
                if hash.contains(ch) {
                    return false;
                }

                hash.push(*ch);
            }

            true
        })
        .map(|p| p + 14)
        .unwrap();

    println!("Answer = {}", answer);
}

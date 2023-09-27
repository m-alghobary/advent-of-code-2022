#![allow(unused)]

use terminal::Terminal;
pub mod terminal;

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match &value[..=3] {
            "$ cd" => Command::Cd(String::from(&value[5..])),
            "$ ls" => Command::Ls,
            _ => panic!("Invalid command"),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut terminal = Terminal::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let command = Command::from(line);

            match &command {
                Command::Cd(path) => terminal.update_cwd(path),
                Command::Ls => {}
            }

            continue;
        }

        // output lines
        let mut line_parts = line.split_ascii_whitespace();
        let prefix = line_parts.next().unwrap();

        if prefix != "dir" {
            let size = prefix.parse::<usize>().unwrap_or(0);

            terminal.update_size(size);
        }
    }

    println!();
    println!("Result = {}", terminal.get_folder_to_delete());
}

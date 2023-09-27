use std::string::ParseError;

enum Instruction {
    Noop,
    Addx(isize),
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            return Ok(Self::Noop);
        }

        let mut instr_parts = value.split_whitespace();
        let instraction = instr_parts.next().unwrap();
        if instraction == "addx" {
            let value = instr_parts.next().unwrap().parse::<isize>().unwrap();
            return Ok(Self::Addx(value));
        }

        Ok(Self::Noop)
    }
}

type BufferPos = (usize, usize);

struct Gpu {
    buffer: [Vec<char>; 6],
}

impl Gpu {
    fn new() -> Self {
        let row = vec!['.'; 40];
        Self {
            buffer: [
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
                row,
            ],
        }
    }

    fn draw(&mut self, ch: char, pos: BufferPos) {
        self.buffer[pos.0].insert(pos.1, ch);
    }
}

struct Cpu {
    x: isize,
    sycles: isize,
    gpu: Gpu,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            sycles: 0,
            gpu: Gpu::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.sycle(),
            Instruction::Addx(x) => {
                self.sycle();
                self.sycle();
                self.x += x;
            }
        }
    }

    fn sycle(&mut self) {
        self.sycles += 1;
        let mut c = self.sycles - 1;

        let row = match self.sycles {
            1..=40 => 0,
            41..=80 => 1,
            81..=120 => 2,
            121..=160 => 3,
            161..=200 => 4,
            201..=240 => 5,
            _ => unreachable!("Too many sycles!"),
        };

        // keep c in [0..39] range
        c = ((row * 40) - c).abs();

        if [self.x - 1, self.x, self.x + 1].contains(&c) {
            self.gpu.draw('#', (row as usize, c as usize))
        }
    }

    fn render(&self) {
        for row in &self.gpu.buffer {
            for ch in row {
                print!("{ch}");
            }
            println!();
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let instractions = input
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect::<Vec<Instruction>>();

    let mut cpu = Cpu::new();
    for instruction in instractions {
        cpu.execute(instruction);
    }

    cpu.render()
}

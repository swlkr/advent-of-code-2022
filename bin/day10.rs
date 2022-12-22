use std::fs::read_to_string;

#[derive(Debug)]
enum Command {
    Addx,
    Noop,
}

struct Instruction {
    command: Command,
    value: Option<i32>,
    cycles: u32,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "noop" => Self {
                command: Command::Noop,
                value: None,
                cycles: 1,
            },
            "addx" => {
                let value = String::from(parts[1]).parse::<i32>().unwrap();
                let instruction = Instruction {
                    command: Command::Addx,
                    value: Some(value),
                    cycles: 2,
                };
                return instruction;
            }
            _ => {
                panic!("Cannot parse instruction from program input");
            }
        }
    }
}

struct Register {
    value: i32,
}

struct Processor {
    register: Register,
    execution: Vec<i32>,
    program_counter: u32,
}

impl Processor {
    fn new() -> Self {
        Self {
            register: Register { value: 1 },
            execution: vec![],
            program_counter: 0,
        }
    }

    fn ready(&self) -> bool {
        self.program_counter == 0
    }

    fn execute(&mut self, instruction: &Option<Instruction>) -> Option<i32> {
        let mut result: Option<i32> = None;
        if let Some(i) = instruction {
            if self.program_counter < i.cycles - 1 {
                self.program_counter += 1;
            } else {
                // execution has finished here
                match i.command {
                    Command::Addx => {
                        result = i.value;
                    }
                    Command::Noop => {}
                }
                self.program_counter = 0;
            }
        }
        return result;
    }

    fn update_register(&mut self, value: Option<i32>) {
        if let Some(val) = value {
            self.register.value += val;
        }
        self.execution.push(self.register.value);
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut s = Self { instructions };
        // reverse instructions to pop
        // face palm
        s.instructions.reverse();
        return s;
    }

    fn next(&mut self) -> Option<Instruction> {
        self.instructions.pop()
    }
}

struct Device {
    processor: Processor,
    screen: Screen,
}

impl Device {
    fn new() -> Self {
        Self {
            processor: Processor::new(),
            screen: Screen::new(240),
        }
    }

    fn run(instructions: Vec<Instruction>) -> Device {
        let mut device = Device::new();
        let mut program = Program::new(instructions);
        let mut instruction: Option<Instruction> = None;
        let mut cycle: usize = 0;
        loop {
            if device.processor.ready() {
                instruction = program.next();
            }
            let result = device.processor.execute(&instruction);
            device.screen.draw(cycle);
            device.processor.update_register(result);
            device.screen.update_sprite(device.processor.register.value);
            if instruction.is_none() {
                break;
            }
            cycle += 1;
        }
        return device;
    }

    fn signal_strength(&self) -> i32 {
        let cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
        let mut strengths: Vec<i32> = vec![];
        for cycle in cycles {
            let index: usize = (cycle as usize) - 2;
            let strength = cycle * self.processor.execution[index];
            strengths.push(strength);
        }
        let signal_strength: i32 = strengths.iter().sum();
        return signal_strength;
    }
}

struct Pixel {
    on: bool,
}

impl Pixel {
    fn new() -> Self {
        Self { on: false }
    }

    fn to_string(&self) -> String {
        let str = match self.on {
            true => "#",
            false => ".",
        };
        return String::from(str);
    }
}

struct Sprite {
    position: (i32, i32, i32),
}

struct Screen {
    pixels: Vec<Pixel>,
    sprite: Sprite,
}
impl Screen {
    fn new(pixels: u32) -> Self {
        Self {
            pixels: (0..=pixels).map(|_| Pixel::new()).collect::<Vec<Pixel>>(),
            sprite: Sprite {
                position: (0, 1, 2),
            },
        }
    }

    fn pixel_within_sprite(&self, sprite: &Sprite, cycle: i32) -> bool {
        sprite.position.0 == cycle || sprite.position.1 == cycle || sprite.position.2 == cycle
    }

    fn current_pixel_index(&self, cycle: usize) -> i32 {
        let result = if cycle >= 200 {
            cycle - 200
        } else if cycle >= 160 {
            cycle - 160
        } else if cycle >= 120 {
            cycle - 120
        } else if cycle >= 80 {
            cycle - 80
        } else if cycle >= 40 {
            cycle - 40
        } else {
            cycle
        };
        return result as i32;
    }

    fn draw(&mut self, cycle: usize) {
        self.pixels[cycle].on =
            self.pixel_within_sprite(&self.sprite, self.current_pixel_index(cycle));
    }

    fn update_sprite(&mut self, value: i32) {
        self.sprite.position.0 = value - 1;
        self.sprite.position.1 = value;
        self.sprite.position.2 = value + 1;
    }

    fn print(&self) {
        let mut i = 0;
        for pixel in &self.pixels {
            print!("{}", pixel.to_string());
            i += 1;
            if i != 0 && i % 40 == 0 {
                // new row
                println!("");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day10.txt")?;
    let program = input
        .lines()
        .map(|line| Instruction::from(line))
        .collect::<Vec<Instruction>>();
    let device = Device::run(program);
    println!("{:?}", device.signal_strength());
    device.screen.print();
    return Ok(());
}

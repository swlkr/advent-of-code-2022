struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new(crates: Vec<char>) -> Self {
        Self { crates }
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    fn pop_n(&mut self, n: usize) -> Vec<char> {
        let mut result = vec![];
        let mut counter = n;
        while counter > 0 {
            if let Some(c) = self.crates.pop() {
                result.push(c);
            }
            counter -= 1;
        }
        result.reverse();
        return result;
    }

    fn push(&mut self, input: char) {
        self.crates.push(input);
    }

    fn top(&self) -> String {
        if let Some(c) = self.crates.last() {
            c.to_string()
        } else {
            " ".to_string()
        }
    }
}

fn part1(input: &String) -> String {
    let mut stacks: Vec<Stack> = vec![
        Stack::new(vec!['Z', 'P', 'M', 'H', 'R']),
        Stack::new(vec!['P', 'C', 'J', 'B']),
        Stack::new(vec!['S', 'N', 'H', 'G', 'L', 'C', 'D']),
        Stack::new(vec!['F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L']),
        Stack::new(vec!['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M']),
        Stack::new(vec!['T', 'F', 'S', 'Z', 'B', 'G']),
        Stack::new(vec!['N', 'R', 'V']),
        Stack::new(vec!['P', 'G', 'L', 'T', 'D', 'V', 'C', 'M']),
        Stack::new(vec!['W', 'Q', 'N', 'J', 'F', 'M', 'L']),
    ];
    for line in input.lines().skip(10) {
        let (mut times, from, to) = moves(line);
        while times > 0 {
            if let Some(c) = stacks[from - 1].pop() {
                stacks[to - 1].push(c);
            }
            times -= 1;
        }
    }
    return stacks
        .iter()
        .map(|x| x.top())
        .collect::<Vec<String>>()
        .join("");
}

fn part2(input: &String) -> String {
    let mut stacks: Vec<Stack> = vec![
        Stack::new(vec!['Z', 'P', 'M', 'H', 'R']),
        Stack::new(vec!['P', 'C', 'J', 'B']),
        Stack::new(vec!['S', 'N', 'H', 'G', 'L', 'C', 'D']),
        Stack::new(vec!['F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L']),
        Stack::new(vec!['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M']),
        Stack::new(vec!['T', 'F', 'S', 'Z', 'B', 'G']),
        Stack::new(vec!['N', 'R', 'V']),
        Stack::new(vec!['P', 'G', 'L', 'T', 'D', 'V', 'C', 'M']),
        Stack::new(vec!['W', 'Q', 'N', 'J', 'F', 'M', 'L']),
    ];
    for line in input.lines().skip(10) {
        let (times, from, to) = moves(line);
        let mut crates = stacks[from - 1].pop_n(times);
        stacks[to - 1].crates.append(&mut crates);
    }
    return stacks
        .iter()
        .map(|x| x.top())
        .collect::<Vec<String>>()
        .join("");
}

fn moves(input: &str) -> (usize, usize, usize) {
    let moves = input
        .split(' ')
        .filter(|part| *part != "move" && *part != "from" && *part != "to")
        .map(|m| m.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return (moves[0], moves[1], moves[2]);
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./bin/day5.txt")?;
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
    return Ok(());
}

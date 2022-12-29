enum Operator {
    Add,
    Multiply,
    Square,
}

struct Inspection {
    index: usize,
    item: u64,
}

struct Monkey {
    items: Vec<u64>,
    inspections: u64,
    operation: (Operator, u64),
    test: u64,
    test_result: (usize, usize),
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: (Operator, u64),
        test: u64,
        test_result: (usize, usize),
    ) -> Self {
        Self {
            items,
            inspections: 0,
            operation,
            test,
            test_result,
        }
    }

    fn inspect(&self, item: u64) -> Inspection {
        let worry_level = self.worry_about(item);
        let final_worry_level = worry_level / 3;
        let monkey_index = if final_worry_level % self.test == 0 {
            self.test_result.0
        } else {
            self.test_result.1
        };
        return Inspection {
            index: monkey_index,
            item: final_worry_level,
        };
    }

    fn inspect_part2(&self, item: u64, common_multiple: u64) -> Inspection {
        let worry_level = self.worry_about(item);
        let worry_level = worry_level % common_multiple;
        let monkey_index = if worry_level % self.test == 0 {
            self.test_result.0
        } else {
            self.test_result.1
        };
        return Inspection {
            index: monkey_index,
            item: worry_level,
        };
    }

    fn worry_about(&self, item: u64) -> u64 {
        match self.operation.0 {
            Operator::Add => item + self.operation.1,
            Operator::Multiply => item * self.operation.1,
            Operator::Square => item * item,
        }
    }
}

fn monkey_business(monkeys: Vec<Monkey>) -> u64 {
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    let monkey_business: u64 = inspections[0] * inspections[1];
    return monkey_business;
}

fn part1(rounds: u64) -> u64 {
    let mut monkeys = monkeys();
    for _ in 0..rounds {
        for mi in 0..monkeys.len() {
            monkeys[mi].items.reverse();
            while let Some(item) = monkeys[mi].items.pop() {
                let inspection = monkeys[mi].inspect(item);
                monkeys[mi].inspections += 1;
                monkeys[inspection.index].items.push(inspection.item);
            }
        }
    }
    return monkey_business(monkeys);
}

fn part2(rounds: u64) -> u64 {
    let mut monkeys = monkeys();
    let common_multiple = monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for mi in 0..monkeys.len() {
            monkeys[mi].items.reverse();
            while let Some(item) = monkeys[mi].items.pop() {
                let inspection = monkeys[mi].inspect_part2(item, common_multiple);
                monkeys[mi].inspections += 1;
                monkeys[inspection.index].items.push(inspection.item);
            }
        }
    }
    return monkey_business(monkeys);
}

fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![57, 58], (Operator::Multiply, 19), 7, (2, 3)),
        Monkey::new(vec![66, 52, 59, 79, 94, 73], (Operator::Add, 1), 19, (4, 6)),
        Monkey::new(vec![80], (Operator::Add, 6), 5, (7, 5)),
        Monkey::new(
            vec![82, 81, 68, 66, 71, 83, 75, 97],
            (Operator::Add, 5),
            11,
            (5, 2),
        ),
        Monkey::new(
            vec![55, 52, 67, 70, 69, 94, 90],
            (Operator::Square, 0),
            17,
            (0, 3),
        ),
        Monkey::new(vec![69, 85, 89, 91], (Operator::Add, 7), 13, (1, 7)),
        Monkey::new(vec![75, 53, 73, 52, 75], (Operator::Multiply, 7), 2, (0, 4)),
        Monkey::new(vec![94, 60, 79], (Operator::Add, 2), 3, (1, 6)),
    ]
}

fn main() {
    let part1 = part1(20);
    println!("{}", part1);
    let part2 = part2(10_000);
    println!("{}", part2);
}

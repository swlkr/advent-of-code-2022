use std::{collections::HashSet, fs::read_to_string};

struct Set {
    chars: Vec<char>,
    marker: usize,
}

impl Set {
    fn new() -> Self {
        Self {
            chars: vec![],
            marker: 0,
        }
    }

    fn push(&mut self, c: char) {
        self.chars.push(c);
        self.marker += 1;
    }

    fn has_min_value(&self) -> bool {
        self.chars.len() >= 4
    }

    fn has_min_value_part2(&self) -> bool {
        self.chars.len() >= 14
    }

    fn is_unique(&self) -> bool {
        let mut uniq = HashSet::new();
        return self.chars.iter().all(|x| uniq.insert(x));
    }

    fn marker_found(&mut self) -> bool {
        if self.has_min_value() && self.is_unique() {
            return true;
        } else {
            if self.has_min_value() {
                self.chars.reverse();
                self.chars.pop();
                self.chars.reverse();
            }
            return false;
        }
    }

    fn marker_found_part2(&mut self) -> bool {
        if self.has_min_value_part2() && self.is_unique() {
            return true;
        } else {
            if self.has_min_value_part2() {
                self.chars.reverse();
                self.chars.pop();
                self.chars.reverse();
            }
            return false;
        }
    }
}

fn part1(input: &String) -> usize {
    let mut set = Set::new();
    for c in input.chars() {
        if set.marker_found() {
            break;
        }
        set.push(c);
    }
    return set.marker;
}

fn part2(input: &String) -> usize {
    let mut set = Set::new();
    for c in input.chars() {
        if set.marker_found_part2() {
            break;
        }
        set.push(c);
    }
    return set.marker;
}
fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day6.txt")?;
    let result = part1(&input);
    println!("{result}");
    let result = part2(&input);
    println!("{result}");
    return Ok(());
}

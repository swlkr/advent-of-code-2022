use std::{collections::HashSet, fs::read_to_string};

fn priority(x: &u8) -> u8 {
    if x.is_ascii_lowercase() {
        x.to_ascii_lowercase() - 96
    } else {
        (x.to_ascii_uppercase() - 65) + 27
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day3.txt")?;
    let mut score: u32 = 0;
    for line in input.lines() {
        // split each line in half
        let len = line.len();
        let halves = line.split_at(len / 2);
        // make each half a HashSet
        let left: HashSet<u8> = HashSet::from_iter(halves.0.as_bytes().to_vec());
        let right: HashSet<u8> = HashSet::from_iter(halves.1.as_bytes().to_vec());
        // return first value from intersection
        if let Some(intersection) = left.intersection(&right).next() {
            let priority = priority(intersection);
            score += u32::from(priority);
        }
    }
    println!("{}", score);

    let mut score2: u32 = 0;
    let groups = input.lines().collect::<Vec<&str>>();
    for group in groups.chunks(3) {
        let first: HashSet<u8> = HashSet::from_iter(group[0].as_bytes().to_vec());
        let second: HashSet<u8> = HashSet::from_iter(group[1].as_bytes().to_vec());
        let third: HashSet<u8> = HashSet::from_iter(group[2].as_bytes().to_vec());
        let mut sets = vec![first, second, third];
        let (intersection, others) = sets.split_at_mut(1);
        let intersection = &mut intersection[0];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }
        if let Some(inter) = intersection
            .clone()
            .into_iter()
            .collect::<Vec<u8>>()
            .first()
        {
            score2 += u32::from(priority(inter));
        }
    }
    println!("{}", score2);
    return Ok(());
}

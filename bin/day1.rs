use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day1.txt")?;
    let mut elves = vec![];
    let mut cals = 0;
    for line in input.lines() {
        if line == "" {
            elves.push(cals);
            cals = 0;
        } else {
            let cal: i32 = line.parse().unwrap();
            cals += cal;
        }
    }
    elves.sort();

    // part 1
    let max_cals = elves.last().unwrap();
    println!("{:?}", max_cals);

    // part 2
    let total_cals: i32 = elves.iter().rev().take(3).sum();
    println!("{:?}", total_cals);
    return Ok(());
}

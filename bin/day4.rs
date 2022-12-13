use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;
    let input = read_to_string("./bin/day4.txt")?;
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let left: Vec<&str> = split[0].split('-').collect();
        let right: Vec<&str> = split[1].split('-').collect();
        let min_left: u32 = left[0].parse().unwrap();
        let max_left: u32 = left[1].parse().unwrap();
        let min_right: u32 = right[0].parse().unwrap();
        let max_right: u32 = right[1].parse().unwrap();

        // full contain situation
        if (min_left <= min_right && max_left >= max_right)
            || (min_right <= min_left && max_right >= max_left)
        {
            part1 += 1;
        }

        // partially contains
        // 5-7,7-9
        // 6-6,4-8
        if ((min_right >= min_left && min_right <= max_left)
            || (max_right >= min_left && max_right <= max_left))
            || ((min_left >= min_right && min_left <= max_right)
                || (max_left >= min_right && max_left <= max_right))
        {
            part2 += 1;
        }
    }
    println!("{}", part1);
    println!("{}", part2);
    return Ok(());
}

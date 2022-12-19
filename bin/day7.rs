use std::{collections::HashMap, fs::read_to_string};

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day7.real.txt")?;
    let mut current_path: Vec<&str> = vec![""];
    let mut fs: HashMap<String, u64> = HashMap::new();
    fs.insert(String::from(""), 0);
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let p = &line[5..];
            match p {
                ".." => match current_path.pop() {
                    Some(_) => {
                        // do nothing
                    }
                    None => {
                        current_path = vec![""];
                    }
                },
                "/" => {
                    current_path = vec![""];
                }
                _ => {
                    current_path.push(p);
                }
            }
        } else if line.starts_with("$ ls") {
            // noop
        } else if line.starts_with("dir") {
            let path = file_join(&current_path);
            if !fs.contains_key(&path) {
                fs.insert(path, 0);
            }
        } else {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let file = (parts[0].parse::<u64>().unwrap(), &parts[1]);
            let path = file_join(&current_path);
            if fs.contains_key(&path) {
                let size = fs.get(&path).unwrap();
                fs.insert(path, size + file.0);
            } else {
                fs.insert(path, file.0);
            }
        }
    }

    let mut sizes: HashMap<String, u64> = HashMap::new();
    for (dir, size) in fs {
        let parts = dir.split("/").collect::<Vec<&str>>();
        let mut i = 0;
        while i < parts.len() {
            let n = parts[0..=i].join("/");
            let s = sizes.get(&n).unwrap_or(&0);
            sizes.insert(n, size + s);
            i += 1;
        }
    }

    let part1: u64 = sizes.values().filter(|x| *x <= &100_000).sum();
    println!("{part1}");

    let disk_size = 70_000_000;
    let free_size = 30_000_000;
    let used_size = sizes[""];
    let needed_size = disk_size - used_size;
    let target_size = free_size - needed_size;

    let part2: &u64 = sizes.values().filter(|x| *x >= &target_size).min().unwrap();
    println!("{part2}");

    return Ok(());
}

fn file_join(paths: &Vec<&str>) -> String {
    let mut result: Vec<&str> = vec![];
    for part in paths {
        result.push(part);
    }
    return result.join("/");
}

use std::{collections::HashSet, fs::read_to_string};

const MAX_DISTANCE: i32 = 2;

struct Coordinate {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Knot {
    positions: Vec<Coordinate>,
}

impl std::fmt::Display for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args = vec![];
        for c in &self.positions {
            args.push(format!("({}, {})", c.x, c.y));
        }
        return f.write_str(args.join(" ").as_str());
    }
}

impl Knot {
    fn new() -> Self {
        Self {
            positions: vec![Coordinate::new(0, 0)],
        }
    }

    fn x(&self) -> i32 {
        self.positions.last().unwrap().x
    }

    fn y(&self) -> i32 {
        self.positions.last().unwrap().y
    }

    // check if two away
    fn should_update(&self, leading_knot: &Knot) -> bool {
        let x_distance = leading_knot.x() - self.x();
        let y_distance = leading_knot.y() - self.y();

        return x_distance.abs() >= MAX_DISTANCE || y_distance.abs() >= MAX_DISTANCE;
    }

    // given a motion and a head knot
    // this function returns a set of coordinates
    // for where the self knot will go
    fn next_coordinate(&self, motion: &Motion, leading_knot: Option<&mut Knot>) -> Coordinate {
        match leading_knot {
            // this is one of the tail knots
            // check if it's two away from the head
            // when determining coordinates
            Some(lead_knot) => {
                let mut coordinate = Coordinate::new(self.x(), self.y());
                if self.should_update(&lead_knot) {
                    // update self position by 1 in either direction
                    if self.x() < lead_knot.x() {
                        coordinate.x = self.x() + 1;
                    } else if self.x() > lead_knot.x() {
                        coordinate.x = self.x() - 1;
                    }

                    if self.y() < lead_knot.y() {
                        coordinate.y = self.y() + 1;
                    } else if self.y() > lead_knot.y() {
                        coordinate.y = self.y() - 1;
                    }
                }
                return coordinate;
            }
            None => {
                // self is the head knot
                // no need to check if it's two away from anything
                match motion.direction {
                    Direction::Down => Coordinate::new(self.x(), self.y() - 1),
                    Direction::Left => Coordinate::new(self.x() - 1, self.y()),
                    Direction::Right => Coordinate::new(self.x() + 1, self.y()),
                    Direction::Up => Coordinate::new(self.x(), self.y() + 1),
                }
            }
        }
    }

    fn move_to(&mut self, motion: &Motion, leading_knot: Option<&mut Knot>) {
        self.positions
            .push(self.next_coordinate(motion, leading_knot));
    }
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(num_knots: i32) -> Self {
        let mut knots = vec![];
        for _ in 0..num_knots {
            knots.push(Knot::new());
        }
        Self { knots }
    }

    fn move_to(&mut self, motion: &Motion) {
        for _ in 0..motion.steps {
            let mut leading_knot: Option<&mut Knot> = None;
            for knot in &mut self.knots {
                knot.move_to(motion, leading_knot);
                leading_knot = Some(knot);
            }
        }
    }

    fn tail_positions(&self) -> usize {
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for c in &self.knots.last().unwrap().positions {
            set.insert((c.x, c.y));
        }
        return set.len();
    }
}

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

struct Motion {
    direction: Direction,
    steps: i32,
}

impl Motion {
    fn new(direction: Direction, steps: i32) -> Self {
        Self { direction, steps }
    }
}

impl From<Vec<&str>> for Motion {
    fn from(parts: Vec<&str>) -> Self {
        let x: i32 = parts[1].parse().unwrap();
        let direction = match parts[0] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Can't move in that direction"),
        };
        return Motion::new(direction, x);
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day9.txt")?;
    let mut rope = Rope::new(2);
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let motion = Motion::from(parts);
        rope.move_to(&motion);
    }
    println!("part 1: {:?}", rope.tail_positions());

    // part 2
    let mut rope = Rope::new(10);
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let motion = Motion::from(parts);
        rope.move_to(&motion);
    }
    println!("part 2: {:?}", rope.tail_positions());
    return Ok(());
}

use std::{collections::HashSet, fs::read_to_string};

struct Grid {
    trees: Vec<Vec<Tree>>,
}

impl Grid {
    fn new(trees: Vec<Vec<Tree>>) -> Self {
        Self { trees }
    }

    fn visible_trees(&self) -> usize {
        let mut vis_trees = HashSet::new();
        let mut row = 1;
        while row < self.trees.len() - 1 {
            let mut col = 1;
            while col < self.trees[row].len() - 1 {
                let tree = &self.trees[row][col];
                let top = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.0 < tree.location.0 && t.location.1 == tree.location.1)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                let bottom = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.0 > tree.location.0 && t.location.1 == tree.location.1)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                let left = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.1 < tree.location.1 && t.location.0 == tree.location.0)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                let right = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.1 > tree.location.1 && t.location.0 == tree.location.0)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                if tree.visible_from(left)
                    || tree.visible_from(right)
                    || tree.visible_from(top)
                    || tree.visible_from(bottom)
                {
                    vis_trees.insert(tree.location);
                }

                col += 1;
            }
            row += 1;
        }
        let outside_trees: usize = self
            .trees
            .iter()
            .flatten()
            .filter(|t| {
                t.location.0 == 0
                    || t.location.1 == 0
                    || t.location.0 == self.row_size() - 1
                    || t.location.1 == self.col_size() - 1
            })
            .count();
        return vis_trees.len() + outside_trees;
    }

    fn scenic_scores(&self) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        let mut row = 1;
        while row < self.trees.len() - 1 {
            let mut col = 1;
            while col < self.trees[row].len() - 1 {
                let tree = &self.trees[row][col];
                let mut top = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.0 < tree.location.0 && t.location.1 == tree.location.1)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                top.reverse();
                let bottom = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.0 > tree.location.0 && t.location.1 == tree.location.1)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                let mut left = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.1 < tree.location.1 && t.location.0 == tree.location.0)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                left.reverse();
                let right = self
                    .trees
                    .iter()
                    .flatten()
                    .filter(|t| t.location.1 > tree.location.1 && t.location.0 == tree.location.0)
                    .map(|x| x.height)
                    .collect::<Vec<u32>>();
                let left_distance = tree.viewing_distance_from(left);
                let right_distance = tree.viewing_distance_from(right);
                let top_distance = tree.viewing_distance_from(top);
                let bottom_distance = tree.viewing_distance_from(bottom);
                let scenic_score = left_distance * right_distance * top_distance * bottom_distance;
                result.push(scenic_score);
                col += 1;
            }
            row += 1;
        }
        return result;
    }

    fn row_size(&self) -> usize {
        self.trees.len()
    }

    fn col_size(&self) -> usize {
        self.trees[0].len()
    }
}

struct Tree {
    height: u32,
    location: (usize, usize),
}

impl Tree {
    fn new(height: u32, location: (usize, usize)) -> Self {
        Self { height, location }
    }

    fn visible_from(&self, heights: Vec<u32>) -> bool {
        return heights.iter().all(|h| self.height > *h);
    }

    fn viewing_distance_from(&self, heights: Vec<u32>) -> u32 {
        let mut result = 0;
        for height in heights {
            if height == self.height {
                result += 1;
                break;
            } else if height < self.height {
                result += 1;
            } else {
                result += 1;
                break;
            }
        }
        return result;
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day8.txt")?;
    let mut grid = Grid::new(vec![]);
    let mut row = 0;
    for line in input.lines() {
        // rows
        let mut trees: Vec<Tree> = vec![];
        let mut col = 0;
        for column in line.chars().map(|c| c.to_string()) {
            //columns
            let tree = Tree::new(column.parse().unwrap(), (row, col));
            trees.push(tree);
            col += 1;
        }
        grid.trees.push(trees);
        row += 1;
    }

    let visible = grid.visible_trees();
    println!("part 1: {}", visible);
    let max_scenic_score: u32 = grid.scenic_scores().iter().fold(0, |a, b| a.max(*b));
    println!("part 2: {}", max_scenic_score);
    return Ok(());
}

use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
struct Tree {
    height: i32,
    visible_north: bool,
    visible_south: bool,
    visible_east: bool,
    visible_west: bool,
    viewing_distance_north: usize,
    viewing_distance_south: usize,
    viewing_distance_east: usize,
    viewing_distance_west: usize,
}
impl Tree {
    pub fn new(height: i32) -> Self {
        Self {
            height,
            visible_north: false,
            visible_south: false,
            visible_east: false,
            visible_west: false,
            viewing_distance_north: 0,
            viewing_distance_south: 0,
            viewing_distance_east: 0,
            viewing_distance_west: 0,
        }
    }
    pub fn scenic_score(&self) -> usize {
        self.viewing_distance_north
            * self.viewing_distance_south
            * self.viewing_distance_east
            * self.viewing_distance_west
    }
}
fn read_forest(input: &str) -> HashMap<(usize, usize), Tree> {
    let mut forest = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, size)| {
            let tree = Tree::new(size.to_string().parse().unwrap());
            forest.insert((x, y), tree);
        });
    });

    forest
}
fn calculate_visibility(forest: &mut HashMap<(usize, usize), Tree>) {
    forest
        .keys()
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect::<Vec<(usize, usize)>>()
        .iter()
        .for_each(|(x, y)| {
            let x_max = forest.keys().fold(
                0 as usize,
                |accum, (x, _)| if *x > accum { *x } else { accum },
            );

            let y_max = forest.keys().fold(
                0 as usize,
                |accum, (_, y)| if *y > accum { *y } else { accum },
            );

            let mut tree = forest.get(&(*x, *y)).unwrap().clone();
            if *x == 0 {
                tree.visible_west = true;
            } else {
                let mut heights = vec![];
                for i in 1..=*x {
                    if let Some(west_tree) = forest.get(&(*x - i, *y)) {
                        heights.push(west_tree.height);
                    }
                }
                if heights.iter().all(|height| tree.height > *height) {
                    tree.visible_west = true;
                    tree.viewing_distance_west = heights.len();
                } else {
                    tree.viewing_distance_west = heights
                        .iter()
                        .take_while(|height| **height < tree.height)
                        .count()
                        + 1;
                }
            }

            if *y == 0 {
                tree.visible_north = true;
            } else {
                let mut heights = vec![];
                for i in 1..=*y {
                    if let Some(north_tree) = forest.get(&(*x, *y - i)) {
                        heights.push(north_tree.height);
                    }
                }
                if heights.iter().all(|height| tree.height > *height) {
                    tree.visible_north = true;
                    tree.viewing_distance_north = heights.len();
                } else {
                    tree.viewing_distance_north = heights
                        .iter()
                        .take_while(|height| **height < tree.height)
                        .count()
                        + 1;
                }
            }

            {
                let mut heights = vec![];
                for i in *x + 1..=x_max {
                    if let Some(east_tree) = forest.get(&(i, *y)) {
                        heights.push(east_tree.height);
                    }
                }
                if heights.iter().all(|height| tree.height > *height) {
                    tree.visible_east = true;
                    tree.viewing_distance_east = heights.len();
                } else {
                    tree.viewing_distance_east = heights
                        .iter()
                        .take_while(|height| **height < tree.height)
                        .count()
                        + 1;
                }
            }

            {
                let mut heights = vec![];
                for i in *y + 1..=y_max {
                    if let Some(south_tree) = forest.get(&(*x, i)) {
                        heights.push(south_tree.height);
                    }
                }
                if heights.iter().all(|height| tree.height > *height) {
                    tree.visible_south = true;
                    tree.viewing_distance_south = heights.len();
                } else {
                    tree.viewing_distance_south = heights
                        .iter()
                        .take_while(|height| **height < tree.height)
                        .count()
                        + 1;
                }
            }

            forest.insert((*x, *y), tree);
        });
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let mut forest = read_forest(&input);
    calculate_visibility(&mut forest);

    let visible_trees = forest.iter().fold(0, |accum, (_, tree)| {
        if tree.visible_north || tree.visible_south || tree.visible_west || tree.visible_east {
            accum + 1
        } else {
            accum
        }
    });

    println!("# of Visible Trees: {visible_trees}");

    let highest_scenic_score = forest.iter().fold(0, |accum, (_, tree)| {
        if tree.scenic_score() > accum {
            tree.scenic_score()
        } else {
            accum
        }
    });

    println!("Highest Scenic Score: {highest_scenic_score}");
}

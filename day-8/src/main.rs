use std::{iter::FromIterator};

const SAMPLE: &str = "30373
25512
65332
33549
35390";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn load_tree_grid(input: String) -> Vec<Vec<u8>> {
    let mut tree_grid = vec![];
    for line in input.lines() {
        tree_grid.push(Vec::from_iter(line.bytes()));
    }
    tree_grid
}

fn is_visible(height: &u8, direction: Vec<&u8>) -> bool {
    for tree in direction {
        if tree >= height {
            return false;
        }
    }
    true   
}

fn calculate_number_visible(tree_grid: &Vec<Vec<u8>>) -> u32 {
    let mut number_visible = 0u32;
    for r in 1..tree_grid[0].len() - 1 {
        for c in 1..tree_grid.len() - 1 {
            let mut visible = true;

            let west = tree_grid[r][0..c].iter().rev().collect::<Vec<_>>();
            visible= is_visible(&tree_grid[r][c], west);

            if visible {
                number_visible += 1;
                continue;
            }

            let east = tree_grid[r][c + 1..tree_grid[0].len()].iter().collect::<Vec<_>>();
            visible = is_visible(&tree_grid[r][c], east);

            if visible {
                number_visible += 1;
                continue;
            }

            let north = tree_grid[0..r].iter().map(|s| &s[c]).rev().collect::<Vec<_>>();
            visible = is_visible(&tree_grid[r][c], north);
            
            if visible {
                number_visible += 1;
                continue;
            }

            let south = tree_grid[r + 1..tree_grid.len()].iter().map(|s| &s[c]).rev().collect::<Vec<_>>();
            visible = is_visible(&tree_grid[r][c], south);

            if visible {
                number_visible += 1;
            }
        }
    }
    number_visible += ((4 * tree_grid.len()) - 4) as u32;
    number_visible
}

fn viewing_distance(height: &u8, direction: Vec<&u8>) -> u32 {
    let mut viewing_distance = 0u32;
    for tree in direction {
        viewing_distance += 1;
        if tree >= height {
            return viewing_distance;
        }
    }
    viewing_distance
}

fn calculate_scenic_score(tree_grid: &Vec<Vec<u8>>) -> u32  {
    let mut scenic_score = 0u32;
    for r in 1..tree_grid[0].len() - 1 {
        for c in 1..tree_grid.len() - 1 {
            let mut viewing_distances = vec![];

            let west = tree_grid[r][0..c].iter().rev().collect::<Vec<_>>();
            viewing_distances.push(viewing_distance(&tree_grid[r][c], west));

            let east = tree_grid[r][c + 1..tree_grid[0].len()].iter().collect::<Vec<_>>();
            viewing_distances.push(viewing_distance(&tree_grid[r][c], east));


            let north = tree_grid[0..r].iter().map(|s| &s[c]).rev().collect::<Vec<_>>();
            viewing_distances.push(viewing_distance(&tree_grid[r][c], north));
            

            let south = tree_grid[r + 1..tree_grid.len()].iter().map(|s| &s[c]).rev().collect::<Vec<_>>();
            viewing_distances.push(viewing_distance(&tree_grid[r][c], south));

            let this_scenic_score = viewing_distances.iter().fold(1u32, |m, d| m * d);
            if this_scenic_score > scenic_score {
                scenic_score = this_scenic_score;
            }
        }
    }
    scenic_score
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let tree_grid = load_tree_grid(input);
    let number_visible = calculate_number_visible(&tree_grid);
    let max_scenic_score = calculate_scenic_score(&tree_grid);
    println!("{}", number_visible);
    println!("{}", max_scenic_score);
}
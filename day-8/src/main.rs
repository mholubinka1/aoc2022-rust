use std::{iter::FromIterator};

mod direction;

use direction::{Direction, find_trees, DIRECTION};

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
    let mut number_visible = ((4 * tree_grid.len()) - 4) as u32;
    for r in 1..tree_grid[0].len() - 1 {
        for c in 1..tree_grid.len() - 1 {
            for direction in DIRECTION {
                let trees = find_trees(tree_grid, r, c, direction);
                if is_visible(&tree_grid[r][c], trees) {
                    number_visible += 1;
                    break;
                }
            }
        }
    }
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
    let mut max_scenic_score = 0u32;
    for r in 1..tree_grid[0].len() - 1 {
        for c in 1..tree_grid.len() - 1 {
            let mut scenic_score = 1u32;
            for direction in DIRECTION {
                let trees = find_trees(tree_grid, r, c, direction);
                scenic_score = scenic_score * viewing_distance(&tree_grid[r][c], trees);
            }
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    max_scenic_score
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
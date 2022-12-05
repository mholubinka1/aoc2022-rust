use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

mod stack;

use stack::{Stack, Move, CreateAllStacks,  MoveCratesIndividually, MoveCrates};

const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}



pub fn find_crates_in_line(s: &str) -> Vec<char> {
    lazy_static! {
        static ref FIND_CRATES: Regex = Regex::new(r"\[A-Z]").unwrap();
    }
    FIND_CRATES.find_iter(s)
        .filter_map(|_crates| _crates.as_str().parse().ok())
        .collect()
}


fn create_stacks_and_define_moves(input: String) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks = Vec::<Stack>::new();
    let mut moves = Vec::<Move>::new();
    let mut stacks_created = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if stacks_created {
            moves.push(Move::from_str(line).unwrap());
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        if !chars.contains(&'[') {
            stacks_created = true;
            continue;
        }  
        let total_stacks = ((chars.len() + 1) / 4) as i32;
        if stacks.len() == 0 {
            stacks.create_all_stacks(total_stacks);
        }
        let mut stack_index = 0;    
        for index in (1..chars.len() - 1).step_by(4) {
            if !chars[index].is_whitespace() {
                stacks[stack_index].insert_below(chars[index]);
            }
            stack_index += 1;
        } 
    }
    (stacks, moves)
}



fn rearrange_stacks_one_at_a_time<'a>(stacks: &'a mut Vec<Stack>, moves: &'a Vec<Move>) -> &'a Vec<Stack> {
    for _move in moves {
        stacks.move_crates_individually(_move);
    }
    stacks
}

fn rearrange_stacks<'a>(stacks: &'a mut Vec<Stack>, moves: &'a Vec<Move>) -> &'a Vec<Stack> {
    for _move in moves {
        stacks.move_crates(_move);
    }
    stacks
}



fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let (stacks, moves) = create_stacks_and_define_moves(input);
    let mut stacks_to_rearrange = stacks;
    //let rearranged_stacks = rearrange_stacks_one_at_a_time(&mut stacks_to_rearrange, &moves);
    let rearranged_stacks = rearrange_stacks(&mut stacks_to_rearrange, &moves);
    let mut crates_at_top = Vec::<char>::new();
    for stack in rearranged_stacks {
        crates_at_top.push(stack.top());
    }
    println!("{:?}", crates_at_top);
}
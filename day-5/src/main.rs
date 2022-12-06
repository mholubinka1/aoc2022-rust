use std::str::FromStr;

mod stack;
use stack::{Stack, Move, CreateAllStacks,  MoveCratesIndividually, MoveCrates};

/*const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";*/

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
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
                stacks[stack_index].insert_at_bottom(chars[index]);
            }
            stack_index += 1;
        } 
    }
    (stacks, moves)
}

fn clone_stacks<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    let newvec = vec.to_vec();
    newvec
}

fn rearrange_stacks_one_at_a_time<'a>(stacks: &'a mut Vec<Stack>, moves: &'a Vec<Move>) -> &'a Vec<Stack> {
    for _move in moves {
        stacks.move_crates_individually(_move);
    }
    stacks
}

fn rearrange_stacks<'b>(stacks: &'b mut Vec<Stack>, moves: &'b Vec<Move>) -> &'b Vec<Stack> {
    for _move in moves {
        stacks.move_crates(_move);
    }
    stacks
}

fn display_crates_on_top(stacks: &Vec<Stack>) -> () {
    let mut crates_at_top = Vec::<char>::new();
    for stack in stacks {
        crates_at_top.push(stack.top());
    }
    println!("{:?}", crates_at_top);
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let (stacks, moves) = create_stacks_and_define_moves(input);
    
    let mut first = clone_stacks(&stacks);
    let first_rearranged_stacks = rearrange_stacks_one_at_a_time(
        &mut first, 
        &moves);

    display_crates_on_top(&first_rearranged_stacks);
    
    let mut second = clone_stacks(&stacks);
    let second_rearranged_stacks = rearrange_stacks(
        &mut second, 
        &moves);
    
    display_crates_on_top(&second_rearranged_stacks);
}

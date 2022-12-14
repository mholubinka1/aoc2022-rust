use std::{str::FromStr, string::ParseError};

mod helpers;

use helpers::str_strip_numbers;

#[derive(Clone)]
pub struct Stack {
    crates: Vec<char>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            crates: Vec::<char>::new(),
        }
    }

    pub fn insert_at_bottom(&mut self, _crate: char) -> () {
        let _ = &self.crates.insert(0, _crate);
    }

    pub fn top(&self) -> char {
        *self.crates.last().unwrap()
    }

    pub fn remove_from_top(&mut self, number: usize) -> () {
        for _ in 0..number {
            let _ = self.crates.pop();
        }
    }
}

pub struct Move {
    from: usize,
    to: usize,
    number: usize
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = str_strip_numbers(s);
        if num.len() != 3 {
            panic!("Move text does not contain 3 numbers: {}", s);
        }
        Ok(Self {
            from: num[1],
            to: num[2],
            number: num[0],
        })
    }
}



pub trait MoveCratesIndividually {
    fn move_crates_individually(&mut self, instruction: &Move) -> ();
}

impl MoveCratesIndividually for Vec<Stack> {
    fn move_crates_individually(&mut self, instruction: &Move) ->  () {
        for _ in 0..instruction.number {
            let _crate = &self[instruction.from - 1].crates.last().unwrap().clone();
            self[instruction.to - 1].crates.push(*_crate);
            let _ = &self[instruction.from - 1].remove_from_top(1);
        }
    }   
}

pub trait MoveCrates {
    fn move_crates(&mut self, instruction: &Move) -> ();
}

impl MoveCrates for Vec<Stack> {
    fn move_crates(&mut self, instruction: &Move) ->  () {
        let mut start_index = self[instruction.from - 1].crates.len() - instruction.number;
        for _ in 0..instruction.number {
            let _crate = &self[instruction.from - 1].crates[start_index].clone();
            let _ = &self[instruction.to - 1].crates.push(*_crate);
            start_index += 1;
        }
        let _ = &self[instruction.from - 1].remove_from_top(instruction.number);
    }   
}

pub trait CreateAllStacks {
    fn create_all_stacks(&mut self, total_stacks: i32) -> ();
}

impl CreateAllStacks for Vec<Stack> {
    fn create_all_stacks(self: &mut Vec<Stack>, total_stacks: i32) -> () {
        for _ in 0..total_stacks{
            let _ = &self.push(Stack::new());
        };
    }
}

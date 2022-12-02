

use std::str::FromStr;

#[derive(Eq, Hash,Debug, PartialEq)]
pub enum Outcome {
    Lose, //X
    Draw, //Y
    Win   //Z
}

#[derive(Eq, Hash,Debug, PartialEq)]
pub enum Move {
    A, //rock
    B, //paper
    C, //scissors
    X, //rock
    Y, //paper
    Z  //scissors
}

impl Move {
    pub fn move_value(&self) -> i64 {
        match *self {
            Move::A => 1,
            Move::B => 2,
            Move::C => 3,
            Move::X => 1,
            Move::Y => 2,
            Move::Z => 3,
        }
    }

    pub fn translate_to_outcome(&self) -> Outcome {
        match *self {
            Move::X => Outcome::Lose,
            Move::Y => Outcome::Draw,
            Move::Z => Outcome::Win,
            _ => unreachable!()
        }
    }

    pub fn drawing_move(&self) -> Move {
        match *self {
            Move::A => Move::X,
            Move::B => Move::Y,
            Move::C => Move::Z,
            _ => unimplemented!()
        }
    }

    pub fn winning_move(&self) -> Move {
        match *self {
            Move::A => Move::Y,
            Move::B => Move::Z,
            Move::C => Move::X,
            _ => unimplemented!()
        }
    }

    pub fn losing_move(&self) -> Move {
        match *self {
            Move::A => Move::Z,
            Move::B => Move::X,
            Move::C => Move::Y,
            _ => unimplemented!()
        }
    }


}

impl FromStr for Move {
    type Err = ();

    fn from_str(_move: &str) -> Result<Move, Self::Err> {
        match _move {
            "A" => Ok(Move::A),
            "B" => Ok(Move::B),
            "C" => Ok(Move::C),
            "X" => Ok(Move::X),
            "Y" => Ok(Move::Y),
            "Z" => Ok(Move::Z),
            &_ => unreachable!(),
        }
    }
}
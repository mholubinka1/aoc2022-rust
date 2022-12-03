mod _move;
mod helpers;

use _move::Move;
use _move::Outcome;
use helpers::circular_subtract;
use std::str::FromStr;

/*const TODAY_SAMPLE: &str = "A Y
B X
C Z";
*/

fn load_input() -> String {
    let input: String = std::fs::read_to_string("input").unwrap();
    input
}

fn create_strategy_guide(input: String) -> Vec<(Move, Move)> {
    let mut strategy_guide: Vec<(Move, Move)> = Vec::new();
    for line in input.lines() {
        let split = line.split(' ').collect::<Vec<&str>>();
        let k = Move::from_str(split[0]).unwrap();
        let v = Move::from_str(split[1]).unwrap();
        strategy_guide.push((k, v));
    }
    strategy_guide
}

fn determine_result_score(_move: &Move, opponent_move: &Move) -> i32 {
    let move_value = _move.move_value();
    let opponent_value = opponent_move.move_value();
    let result = circular_subtract(move_value, opponent_value);
    let result_score: i32 = match result.signum() {
        -1 => 0,
        0 => 3,
        1 => 6,
        _ => unreachable!(),
    };
    result_score
}

fn calculate_first_score(strategy_guide: &Vec<(Move, Move)>) -> i32 {
    let mut total_score: i32 = 0;
    for _tuple in strategy_guide {
        let your_move = &_tuple.1;
        let opponent_move = &_tuple.0;
        let move_score = your_move.move_value() + determine_result_score(&your_move, &opponent_move);
        total_score += move_score;
    }
    total_score
}

fn determine_move_from_outcome(outcome: &Outcome, opponent_move: &Move) -> Move {
    let your_move = match outcome {
        Outcome::Draw => opponent_move.drawing_move(),
        Outcome::Win =>  opponent_move.winning_move(),
        Outcome::Lose => opponent_move.losing_move(),
    };
    your_move
}

fn calculate_second_score(strategy_guide: &Vec<(Move, Move)>) -> i32 {
    let mut total_score: i32 = 0;
    for _tuple in strategy_guide {
        let outcome = &_tuple.1.translate_to_outcome();
        let opponent_move = &_tuple.0;
        let your_move = determine_move_from_outcome(&outcome, &opponent_move);
        let move_score = your_move.move_value() + determine_result_score(&your_move, &opponent_move);
        total_score += move_score;
    }
    total_score
}

fn main() {
    let input = load_input();
    let strategy_guide = create_strategy_guide(input);
    //part 1
    let first_score = calculate_first_score(&strategy_guide);
    //part 2
    let second_score = calculate_second_score(&strategy_guide);
    println!("{}", first_score);
    println!("{}", second_score);
}


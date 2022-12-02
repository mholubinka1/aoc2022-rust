const TODAY_SAMPLE: &str = "
";

fn load_input() -> String {
    std::fs::read_to_string("../input").unwrap()
}

fn main() {
    let input = load_input();
    println!("{}", TODAY_SAMPLE);
    println!("{}", input);
}

/*use std::collections::HashMap;

const TODAY_SAMPLE: &str = "
A Y
B X
C Z
";

fn load_strategy_guide() -> String {
    std::fs::read_to_string("../input").unwrap()
}

fn create_score_map() -> HashMap<'static &str, i64> {
    let mut score_map: HashMap<'static &str, i64> = HashMap::new();
    score_map.insert("A X", 4);
    score_map.insert("A Y", 8);
    score_map.insert("A Z", 3);
    score_map.insert("B X", 1);
    score_map.insert("B Y", 5);
    score_map.insert("B Z", 9);
    score_map.insert("C X", 7);
    score_map.insert("C Y", 2);
    score_map.insert("C Z", 6);
    score_map
}

fn calculate_score(String: input, HashMap<'static &str, i64>: score_map) -> i64 {
    let mut tot_score = 0;
    for line in input.lines() {
        let score =  score_map.get(line);
        tot_score += score;
    }
    tot_score
}

fn main() {
    let strategy_guide = load_strategy_guide();
    let score_map = create_score_map();
    let final_score = calculate_score(input, score_map)
    println!("{}", TODAY_SAMPLE);
    println!("{}", input);
}*/
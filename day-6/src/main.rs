const SAMPLE: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn has_dupe<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i-1]) {
            return true;
        }
    }
    false
}

fn find_marker(signal: &Vec<char>, num: usize) -> usize {
    for i in num..signal.len() {
        let snippet = &signal[i-num..i];
        if !has_dupe(snippet) {
            return i;
        }
    }
    unreachable!();
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let signal: Vec<char> = input.chars().collect();
    let marker_index = find_marker(&signal, 4);
    let start_index = find_marker(&signal, 14);
    println!("{}", input);
    println!("{}", marker_index);
    println!("{}", start_index);
}
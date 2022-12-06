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

fn find_first_marker(signal: &Vec<char>) -> usize {
    for i in 4..signal.len() {
        let snippet = &signal[i-4..i];
        if !has_dupe(snippet) {
            return i;
        }
    }
    unreachable!();
}

fn find_start_of_message_marker(signal: &Vec<char>) -> usize {
    for i in 14..signal.len() {
        let snippet = &signal[i-14..i];
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
    let marker_index = find_first_marker(&signal);
    let start_index = find_start_of_message_marker(&signal);
    println!("{}", input);
    println!("{}", marker_index);
    println!("{}", start_index);
}
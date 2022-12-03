const SAMPLE: &str = "
";

fn load_input() -> String {
    let input std::fs::read_to_string("../input").unwrap();
    return input;
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    println!("{}", TODAY_SAMPLE);
    println!("{}", input);
}
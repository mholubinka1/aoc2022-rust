const SAMPLE: &str = "
";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    println!("{}", SAMPLE);
    println!("{}", input);
}
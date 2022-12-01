const today_sample: &str = "
";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn main() {
    let input = load_input();
    println!("{}", today_sample);
    println!("{}", input);
}
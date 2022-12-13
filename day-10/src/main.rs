/*const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";*/

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn parse_instruction(input_line: &str) -> Vec<i32> {
    let split = input_line.trim().split_once(' ');
    match split  {
        Some(("addx", number)) => vec![0, number.parse::<i32>().unwrap()],
        None => vec![0],
        _ => unreachable!(),
    }
}

fn load_instructions(input: String) -> Vec<i32> {
    let mut instructions = vec![];
    for line in input.lines() {
        for el in parse_instruction(line) {
            instructions.push(el);
        }
    }
    instructions
}

fn calculate_total_signal_strength(instructions: &Vec<i32>) -> i32 {
    let starting_x = 1i32;
    let mut total_signal_strength = 0i32;
    for i in (19..instructions.len()).step_by(40) {
        let sum: i32 = instructions[0..i].iter().sum();
        let signal_strength = (starting_x + sum) * ((i + 1) as i32);
        total_signal_strength +=  signal_strength;
    }
    total_signal_strength
}

fn render_image(instructions: &Vec<i32>) -> String {
    let mut crt = String::new();
    for c in 1..instructions.len() + 1 {
        let pos = ((c - 1) % 40) as i32;
        if pos == 0 {
            crt.push('\n');
        }
        let x: i32 = instructions[0..c-1].iter().sum();
        let reg = x + 1;
        let pixel = if (reg - pos).abs() <= 1 {'o'} else {' '};
        crt.push(pixel);
    }
    crt
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let instructions = load_instructions(input);
    let total_signal_strength = calculate_total_signal_strength(&instructions);
    let crt = render_image(&instructions);
    println!("{}", total_signal_strength);
    println!("{}", crt);
}

/* 
ooo  o  o ooo    oo ooo  ooo  o     oo  
o  o o o  o  o    o o  o o  o o    o  o 
o  o oo   o  o    o ooo  o  o o    o  o 
ooo  o o  ooo     o o  o ooo  o    oooo 
o o  o o  o    o  o o  o o    o    o  o 
o  o o  o o     oo  ooo  o    oooo o  o 
*/
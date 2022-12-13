use monkey::Monkey;

use crate::monkey::parse_monkeys;

mod monkey;

const SAMPLE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn perform_rounds(monkeys: &mut Vec<Monkey>, number: usize) -> () {
    let mod_product = monkeys.iter().fold(1, |x, m| x * m.modulus);
    for round in 1..number + 1 {
        for m in 0..monkeys.len() {
            for index in 0..monkeys[m].items() {
                let inspected_item = monkeys[m].inspect_item(index, mod_product);
                let t = monkeys[m].test_item(inspected_item);
                monkeys[t].catch_item(inspected_item);
            }
            monkeys[m].throw_items();
        }
        print!("End of Round {}\n", round)
    }
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();
    let monkey_business = monkeys[0..2].iter().rev().fold(1, |x, m| x * m.inspections);
    println!("{}", monkey_business);
}

fn main() {
    let input = SAMPLE.to_string();
    let input = load_input();
    let mut monkeys = parse_monkeys(input);
    perform_rounds(&mut monkeys, 10000);
}
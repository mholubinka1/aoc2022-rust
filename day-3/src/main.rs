use std::collections::HashSet;

mod rucksack;
use rucksack::Rucksack;

/*const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";*/

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn load_input() -> String {
    let input = std::fs::read_to_string("input").unwrap();
    return input;
}

fn fill_rucksacks(input: String) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in input.lines() {
        let len = line.chars().count();
        if len % 2 != 0 {
            panic!("Number of items is not even: {}. {} ", len, line);
        }
        let slice = len/2;
        let compartment_one = &line[0..slice];
        let compartment_two = &line[slice..];
        rucksacks.push(Rucksack::new(
            compartment_one.to_string(), 
            compartment_two.to_string()
        ));
    }
    rucksacks
}

fn calculate_total(rucksacks: &Vec<Rucksack>, priorities: &Vec<char>) -> i32 {
    let mut tot: i32 = 0;
    for rucksack in rucksacks {
        let common_item = rucksack.find_common_item();
        let priority = priorities.iter().position(|&c| c == common_item).unwrap();
        tot += priority as i32 + 1;
    }
    tot
}

fn find_elf_group_common_item(elves: &[Rucksack]) -> char {
    let mut common_items = elves[0].all_items();
    for i in [1, elves.len() - 1] {
        let items = elves[i].all_items();
        common_items = common_items.intersection(&items).map(|x| *x).collect::<HashSet<_>>();
    }
    if common_items.len() != 1 {
        panic!("More than one common item: {:?}.", common_items);
    }
    let item = Vec::from_iter(common_items)[0];
    item
}

fn calculate_authenticity_total(rucksacks: &Vec<Rucksack>, priorities: &Vec<char>) -> i32 {
    let mut tot = 0;
    for elf_group in rucksacks.chunks(3) {
        if elf_group.len() != 3 {
            panic!("Elf group does not have 3 members.");
        }
        let common_item = find_elf_group_common_item(elf_group);
        let priority = priorities.iter().position(|c| *c == common_item).unwrap();
        tot += priority as i32 + 1;
    }
    tot
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    //println!("{}", input);
    let priorities = ALPHABET.chars().collect();
    let rucksacks = fill_rucksacks(input);
    let total = calculate_total(&rucksacks, &priorities); 
    let authenticity_total = calculate_authenticity_total(&rucksacks, &priorities);
    println!("The priority total for all the individual common items is {}", total);
    println!("The priority total for all the group common items is {}", authenticity_total);
}
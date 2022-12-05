mod assignment;

use std::str::FromStr;

use assignment::ElfAssignment;

const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn create_assignment_pairs(input: String) -> Vec<(ElfAssignment, ElfAssignment)> {
    let mut assignment_pairs = Vec::<(ElfAssignment, ElfAssignment)>::new();
    for line in input.lines() {
        let assignments = line.split(',').flat_map(ElfAssignment::from_str);
        let [x, y]: [ElfAssignment] = assignments.collect::<Vec<_>>()[..] else { unreachable!() };
        assignment_pairs.push((x, y));
    }
    assignment_pairs
}

fn count_containments(assignment_pairs: &Vec<(ElfAssignment, ElfAssignment)>) -> i32 {
    let mut count = 0;
    for pair in assignment_pairs {
        if ElfAssignment::contains(&pair.0, &pair.1) || ElfAssignment::contains(&pair.1, &pair.0) {
            count += 1;
        }
    }
    count
}

fn count_overlaps(assignment_pairs: &Vec<(ElfAssignment, ElfAssignment)>) -> i32 {
    let mut count = 0;
    for pair in assignment_pairs {
        if ElfAssignment::overlaps(&pair.0, &pair.1) || ElfAssignment::overlaps(&pair.1, &pair.0) {
            count += 1;
        }
    }
    count
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let assignment_pairs = create_assignment_pairs(input);
    let containment_count = count_containments(&assignment_pairs);
    let overlap_count = count_overlaps(&assignment_pairs);
    println!("{}", containment_count);
    println!("{}", overlap_count);
}
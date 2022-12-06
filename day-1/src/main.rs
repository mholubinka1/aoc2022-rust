mod elf;

use elf::Elf;

fn load_elves() -> Vec<Elf> {
    let input: String = std::fs::read_to_string("../input").unwrap();
    let mut elves: Vec<Elf> = Vec::new();
    let mut tot_cals: i32 = 0;
    let mut elf_count: i32 = 1;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(Elf::new(elf_count, tot_cals));
            tot_cals = 0;
            elf_count += 1;
        } else {
            tot_cals += line.parse::<i32>().unwrap()
        }
    }
    elves.push(Elf::new(elf_count, tot_cals));
    elves
}

fn main() {
    let mut _elves: Vec<Elf> = load_elves();
    _elves.sort_by_key(|x: &Elf| x.calories);
    _elves.reverse();
    //solution to part 1
    println!("Elf:{} | Carrying {} calories.",_elves[0].number, _elves[0].calories);
    //solution to part 2
    let sum_top_3: i32 = _elves[..3].iter().map(|x: &Elf| x.calories).sum();
    println!("Top 3 elves are carrying {} calories.", sum_top_3);
}
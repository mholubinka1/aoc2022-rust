struct Elf {
    number: i64,
    calories: i64,
}

impl Elf {
    pub fn new(number: i64, calories: i64) -> Self {
        Elf {
            number,
            calories
        }
    }
}

fn load_elves() -> Vec<Elf> {
    let input: String = std::fs::read_to_string("../input").unwrap();
    let mut elves: Vec<Elf> = Vec::new();
    let mut tot_cals: i64 = 0;
    let mut elf_count: i64 = 1;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(Elf::new(elf_count, tot_cals));
            tot_cals = 0;
            elf_count += 1;
        } else {
            tot_cals += line.parse::<i64>().unwrap()
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
    let sum_top_3: i64 = _elves[..3].iter().map(|x: &Elf| x.calories).sum();
    println!("Top 3 elves are carrying {} calories.", sum_top_3);
}
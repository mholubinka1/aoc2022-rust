use std::collections::HashSet;

#[derive(Eq, Hash, Debug, PartialEq)]
pub struct Rucksack {
    compartment_one: String,
    compartment_two: String,
}

impl Rucksack {
    pub fn new(compartment_one: String, compartment_two: String) -> Self {
        Rucksack {
            compartment_one,
            compartment_two
        }
    }

    pub fn find_common_item(&self) -> char {
        let set_one: HashSet<char> = self.compartment_one.chars().collect();
        let set_two: HashSet<char> = self.compartment_two.chars().collect();
        let common = set_one.intersection(&set_two).map(|x| *x).collect::<Vec<_>>();
        if common.len() != 1 {
            panic!("More than one common item: {:?}.", common);
        }
        println!("{}", common[0]);
        common[0]
    }
    
    pub fn all_items(&self) -> HashSet<char> {
        let mut set: HashSet<char> = self.compartment_one.chars().collect();
        let set_two: HashSet<char> = self.compartment_two.chars().collect();
        set.extend(&set_two);
        set
    }
}
pub(crate) struct Elf {
    pub number: i32,
    pub calories: i32,
}

impl Elf {
    pub fn new(number: i32, calories: i32) -> Self {
        Elf {
            number,
            calories
        }
    }
}
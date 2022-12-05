use std::{str::FromStr, string::ParseError};

#[derive(Debug, Copy, Clone)]
pub struct ElfAssignment {
    start: i32,
    end: i32
}

impl ElfAssignment {
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

impl FromStr for ElfAssignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<_> = s.split('-').flat_map(i32::from_str).collect();
        let [start, end] = &range[..] else { unreachable!()} ;
        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}
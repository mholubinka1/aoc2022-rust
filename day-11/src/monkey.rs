use regex::Regex;
use std::mem::take;


pub struct Monkey {
    number: usize,
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    pub modulus: usize,
    pub inspections: usize,
}

impl Monkey {
    pub fn new(
        number: usize, 
        items: Vec<usize>,
        operation: Box<dyn Fn(usize) -> usize>,
        test: Box<dyn Fn(usize) -> usize>,
        modulus: usize) -> Self {
        Self {
            number,
            items,
            operation,
            test,
            inspections: 0,
            modulus,
        }
    }

    pub fn items(&self) -> usize {
        self.items.len()
    }

    pub fn inspect_item(&mut self, i: usize, modulus: usize) -> usize {
        self.inspections += 1;
        let new = (self.operation)(self.items[i]);
        return new % modulus;
    }

    pub fn test_item(&self, item: usize) -> usize {
        (self.test)(item)
    }

    pub fn throw_items(&mut self) -> () {
        self.items = vec![];
    }

    pub fn catch_item(&mut self, thrown_item: usize) -> () {
        self.items.push(thrown_item);
    }
}

pub fn parse_items(line: &str) -> Vec<usize> {
    let split = line.split_once(':');
    match split {
        Some((_, s)) => s.split(',')
                            .filter_map(|s| s.trim().parse::<usize>().ok())
                            .collect::<Vec<_>>(),
        None => unreachable!(),
    }
}

pub fn parse_operation(operation_string: String) -> Box<dyn Fn(usize) -> usize> {
    let split = operation_string.split_once('=');
    match split {
        Some((_, o)) => {
            let operator = o.trim().chars().collect::<Vec<_>>()[4];
            let num = o.split(operator).last().unwrap().trim().parse::<usize>().ok();
            match num {
                Some(y) => Box::new(move |x| {
                    match operator {
                        '*' => x * y,
                        '+' => x + y,
                        _ => panic!("unknown operator"),
                    }
                }),
                None => Box::new(move |x| {
                    match operator {
                        '*' => x * x,
                        '+' => x + x,
                        _ => panic!("unknown operator"),
                    }
                }),
            }        
        }
        None => panic!("could not create dynamic function for monkey"),   
    }
}

pub fn parse_test(numbers: Vec<usize>) -> Box<dyn Fn(usize) -> usize> {
    if numbers.len() != 3 { panic!(); }
    Box::new(move |x| {
        if x % numbers[0] as usize == 0 {
            numbers[1]
        } else {
            numbers[2]
        }
    })
}

pub fn parse_monkeys(input: String) -> Vec<Monkey> {
    let mut i = 0;
    let mut monkeys = Vec::new();

    let mut items = Vec::new();
    let mut operation_string = String::new();
    let mut test_nums =Vec::new();
    let mut modulus = 0usize;

    let re = Regex::new(r"\d+").unwrap();

    for line in input.lines() {
        if line.trim().is_empty() {
            monkeys.push(Monkey::new(
                i, 
                take(&mut items),
                parse_operation(take(&mut operation_string)), 
                parse_test(take(&mut test_nums)),
                modulus,
                ) 
            );
            i += 1;
            continue;
        }
        if line.contains("Starting") {
            items = parse_items(line);
            continue;
        }
        if line.contains("Operation") {
            let split = line.split_once(' ');
            match split {
                Some((_, op)) => operation_string = op.trim().to_string(),
                None => unreachable!(),
            };
            continue;
        }
        if line.contains("Test") || line.contains("true") || line.contains("false") {
            let num = re
                .find_iter(line)
                .filter_map(|digits| digits.as_str().parse::<usize>().ok())
                .collect::<Vec<_>>();
            if num.len() != 1 { panic!() };
            if line.contains("Test") { modulus = num[0]; }
            test_nums.push(num[0]);
            continue;
        }
    }
    monkeys.push(Monkey::new(
        i, 
        take(&mut items),
        parse_operation(take(&mut operation_string)), 
        parse_test(take(&mut test_nums)),
        modulus,
        ) 
    );
    monkeys
}


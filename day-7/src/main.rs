



mod directory;

use directory::{Root, Directory};

const SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

enum Command<'a> {
    ChangeDirectory(&'a str),
    Directory(&'a str),
    File((&'a str, u32)),
}

fn parse_command(input_line: &'_ str) -> Option<Command> {
    match input_line.split_once(' ') {
        Some(("$", command)) => match command.split_once(' ') {
            Some(("cd", dir)) => Some(Command::ChangeDirectory(dir)),
            _ => None,
        },
        Some(("dir", name)) => Some(Command::Directory(name)),
        Some((size, name)) => Some(Command::File((
            name,
            size.parse::<u32>().unwrap(),
        ))),
        _ => unreachable!(),
    }
}


fn create_directories(input: String) -> Root<u32> {
    let mut root = Root { directories: vec![] };
    let mut directory_index: usize = 0;
    root.insert(Directory::new(directory_index, "/", 0u32));

    for line in input.lines() {
        let command = parse_command(line);
        match command {
            Some(Command::ChangeDirectory(dir)) => match dir {
                "/" => directory_index = 0,
                ".." => {
                    match root.parent(directory_index) {
                        Some(parent_index) => {
                            directory_index = parent_index;
                        }
                        None => unreachable!(),
                    }
                }
                dir => {
                    match root.child(directory_index, dir) {
                        Some(child_index) => {
                            directory_index = child_index;
                        }
                        None => unreachable!(),
                    };
                }
            },
            Some(Command::Directory(dir)) => {
                let new_directory_index = root.directories.len();
                let mut directory = Directory::new(
                    new_directory_index,
                    dir,
                    0u32
                );
                directory.parent = Some(directory_index);
                root.insert(directory);
                root.directories[directory_index].children.push(new_directory_index);
            },
            Some(Command::File((_, file_size))) => {
                root.add_file(directory_index, file_size)
            },
            None => {}
        }
    }
    root
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let root = create_directories(input);
    let directory_size_threshold = 100000;
    let result = root
        .directories.iter()
        .filter(|directory| directory.size <= directory_size_threshold)
        .fold(0u32, |x, y| x + y.size);

    let file_system_size = 70000000;
    let required_space = 30000000;
    let root_size = root.directories[0].size;
    let space_to_free = required_space - (file_system_size - root_size);
    
    let mut possible_directories = root
        .directories.iter()
        .filter(|directory| directory.size >= space_to_free)
        .collect::<Vec<_>>();

    possible_directories.sort_by(|x, y| y.size.cmp(&x.size));

    let result = possible_directories.pop().unwrap().size;
    println!("{}", result);
}
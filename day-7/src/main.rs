mod directory;

use directory::{FileSystem, Directory};

/*const SAMPLE: &str = "$ cd /
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
";*/

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

enum Command<'a> {
    ChangeDirectory(&'a str),
    Directory(&'a str),
    File((&'a str, i32)),
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
            size.parse::<i32>().unwrap(),
        ))),
        _ => unreachable!(),
    }
}


fn create_directories(input: String) -> FileSystem<i32> {
    let mut file_system = FileSystem { directories: vec![] };
    let mut directory_index: usize = 0;
    file_system.insert(Directory::new(directory_index, "/", 0i32));

    for line in input.lines() {
        let command = parse_command(line);
        match command {
            Some(Command::ChangeDirectory(dir)) => match dir {
                "/" => directory_index = 0,
                ".." => {
                    match file_system.get_parent(directory_index) {
                        Some(parent_index) => {
                            directory_index = parent_index;
                        }
                        None => unreachable!(),
                    }
                }
                dir => {
                    match file_system.get_child(directory_index, dir) {
                        Some(child_index) => {
                            directory_index = child_index;
                        }
                        None => unreachable!(),
                    };
                }
            },
            Some(Command::Directory(dir)) => {
                let new_directory_index = file_system.directories.len();
                let mut directory = Directory::new(
                    new_directory_index,
                    dir,
                    0i32
                );
                directory.parent = Some(directory_index);
                file_system.insert(directory);
                file_system.directories[directory_index].children.push(new_directory_index);
            },
            Some(Command::File((_, file_size))) => {
                file_system.add_file(directory_index, file_size)
            },
            None => {}
        }
    }
    file_system
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let root = create_directories(input);
    let directory_size_threshold = 100000;
    let result = root
        .directories.iter()
        .filter(|directory| directory.size <= directory_size_threshold)
        .fold(0i32, |x, y| x + y.size);
    println!("{}", result);

    let file_system_size = 70000000;
    let required_space = 30000000;
    let root_size = root.directories[0].size;
    let space_to_free = required_space - (file_system_size - root_size);
    
    let mut possible_directories = root
        .directories.iter()
        .filter(|directory| directory.size >= space_to_free)
        .collect::<Vec<_>>();

    possible_directories.sort_by_key(|x| x.size);
    let index = &possible_directories[0].index();
    println!("{}", index);
    println!("{}", possible_directories[0].size);
}
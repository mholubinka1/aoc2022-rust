use std::ops::AddAssign;

#[derive(Debug, Default)]
pub struct FileSystem<T> where T: PartialEq {
    pub directories: Vec<Directory<T>>,
}

impl<T> FileSystem<T> where T: Copy + PartialEq + AddAssign {
    pub fn insert(&mut self, directory: Directory<T>) {
        self.directories.push(directory);
    }

    pub fn get_parent(&self, index: usize) -> Option<usize> {
        self.directories[index].parent
    }

    pub fn get_child(&self, current_index: usize, target_name: &str) -> Option<usize> {
        self.directories[current_index]
            .children.iter()
            .find(|child_index| self.directories[**child_index].name == target_name).copied()
    }

    pub fn add_file(&mut self, index: usize, file_size: T) {
        self.directories[index].size += file_size;
        if let Some(index) = self.directories[index].parent {
            self.add_file(index, file_size);
        }
    }
}

#[derive(Debug)]
pub struct Directory<T> where T: PartialEq {
    index: usize,
    name: String,
    pub size: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Directory<T> where T: PartialEq {
    pub fn new(index: usize, name: &str, size: T) -> Self {
        Self{
            index,
            name: name.to_string(),
            size,
            parent: None,
            children: vec![],
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}


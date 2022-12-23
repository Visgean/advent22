use std::fs;


struct File {
    name: String,
    size: usize,
}

struct Node {
    path: String,
    folder_name: String,
    files: Vec<File>,
    folders: Vec<Node>,
}

impl Node {
    pub fn get_size(&self) -> usize {
        let mut size = 0;

        for file in self.files.iter() {
            size += file.size;
        }
        for folder in self.folders.iter() {
            size += folder.get_size();
        }
        size
    }

}

pub fn day7() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day7small").expect("Where file");


    for line in contents.lines() {

    }






    Some(0)
}












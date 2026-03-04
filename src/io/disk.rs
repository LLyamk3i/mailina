use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    let directory = PathBuf::from("./storage");
    if !directory.exists() {
        fs::create_dir_all(&directory).expect("failed creating directory");
    }
    directory
}

pub fn read(name: &str) -> String {
    let path = root().join(name);
    fs::read_to_string(path).unwrap_or_else(|_| String::from("{}"))
}

pub fn write(name: &str, content: &str) {
    let path = root().join(name);
    fs::write(path, content).expect("failed writing file");
}

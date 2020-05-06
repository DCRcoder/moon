use std::path::PathBuf;
use std::fs;

static ROOT_DIR_NAME: &str = ".moon";
static TODO_FILE: &str = ".todo.txt";
static DONE_FILE: &str = ".done.txt";

#[derive(PartialEq, Debug, Clone)]
pub struct Config{
    pub root_dir: PathBuf, 
    pub todo_file: PathBuf,
    pub done_file: PathBuf,
}

impl Config {
    pub fn new(current_home_dir: PathBuf) -> Config {
        let home_dir = current_home_dir.join(ROOT_DIR_NAME);
        if !home_dir.exists() {
            fs::create_dir_all(&home_dir).unwrap();
        }
        let todo_file = home_dir.join(TODO_FILE);
        if !todo_file.exists() {
            fs::File::create(&todo_file).unwrap();
        }
        let done_file = home_dir.join(DONE_FILE);
        if !done_file.exists() {
            fs::File::create(&done_file).unwrap();
        }
        return Config{
            root_dir: home_dir,
            todo_file: todo_file,
            done_file: done_file,
        }
    }
}
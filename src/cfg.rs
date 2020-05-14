use std::fs;
use std::path::PathBuf;

static ROOT_DIR_NAME: &str = ".moon";
static TODO_FILE: &str = ".todo.txt";
static DONE_FILE: &str = ".done.txt";

#[derive(Debug, Clone)]
pub struct Config {
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
        return Config {
            root_dir: home_dir,
            todo_file: todo_file,
            done_file: done_file,
        };
    }

    pub fn set_bak(&mut self, mut file_path: PathBuf) {
        file_path.set_file_name((TODO_FILE.to_owned() + ".bak"));
    }

    pub fn set_bak_recover(&mut self, file_path: PathBuf) {
        let bak = file_path.join(".bak");
        let r = fs::rename(bak, file_path);
        if r.is_err() {
            error!("[Config set_bak] error:{:?}", r.err());}
        
    }
}

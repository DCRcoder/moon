use crate::cfg::{Config, TODO_FILE};
use crate::consts::{CREATED_AT, DEFAULT_TODO_PRI_LEVEL, DONE_MESSAGE, TODO_MESSAGE, TODO_PRI};
use crate::error::Result;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, LineWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::str;
extern crate chrono;

use chrono::prelude::*;

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

pub struct Moon {
    pub config: Config,
    todo_writer: BufWriterWithPos<File>,
    done_writer: BufWriterWithPos<File>,
    pub line_count: u64,
    index_map: BTreeMap<u64, u64>,
}

impl Moon {
    pub fn new(cfg: Config) -> Result<Moon> {
        let todo_writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&cfg.todo_file)?,
        )?;
        let done_writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&cfg.done_file)?,
        )?;
        let reader = BufReader::new(OpenOptions::new().read(true).open(&cfg.todo_file)?);
        let mut line_count: u64 = 0;
        let mut index_map = BTreeMap::new();
        let mut pos: u64 = 0;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    line_count += 1;
                    index_map.insert(line_count, pos);
                    pos += line.len() as u64;
                }
                Err(e) => error!("read error: {:?}", e),
            }
        }
        return Ok(Moon {
            todo_writer: todo_writer,
            line_count: line_count,
            config: cfg,
            index_map: index_map,
            done_writer: done_writer,
        });
    }

    pub fn add(&mut self, todo: &str, is_todo: bool) -> io::Result<usize> {
        let local: DateTime<Local> = Local::now();
        let now = local.format("%Y-%m-%d %H:%M:%S").to_string();
        let content = todo.replace("\n", "");
        if is_todo {
            let t = format!(
                "{}:{}|{}:{}|{}:{}\n",
                TODO_PRI, DEFAULT_TODO_PRI_LEVEL, TODO_MESSAGE, content, CREATED_AT, now
            );
            return self.todo_writer.write(t.as_bytes());
        } else 
        {
            let t = format!("{}:{}|{}:{}\n", DONE_MESSAGE, content, CREATED_AT, now);
            return self.done_writer.write(t.as_bytes());
        }
    }

    pub fn list(&mut self, show: &str) {
        let tmpp: PathBuf;
        if show == "done" {
            tmpp = self.config.done_file.clone();
        } else {
            tmpp = self.config.todo_file.clone();
        }

        let reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(tmpp)
                .unwrap(),
        );

        let mut idx = 1;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if show == "todo" {
                        info!("{} todo_content: {}, len:{}", idx, line, line.len());
                    } else {
                        info!("{} done_content: {}, len:{}", idx, line, line.len());
                    }
                    idx += 1;
                }
                Err(e) => error!("read error: {:?}", e),
            }
        }
    }

    pub fn del(&mut self, line_num: u64) {
        let reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(&self.config.todo_file)
                .unwrap(),
        );
        let bak_todo = format!("{}{}", self.config.todo_file.to_str().unwrap(), ".bak");
        info!("[del cmd]bak_todo_path: {}", bak_todo);
        let mut bak_path = PathBuf::from(bak_todo);
        let mut writer = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&bak_path)
            .unwrap();
        for (idx, line) in reader.lines().enumerate() {
            if (idx + 1) as u64 == line_num {
                continue;
            }
            match line {
                Ok(line) => {
                    println!("todo content: {}", line);
                    let content = format!("{}\n", line);
                    writer.write(content.as_bytes()).unwrap();
                }
                Err(e) => {
                    error!("[del cmd] error: {:?}", e);
                }
            }
        }
        writer.flush().unwrap();
        std::fs::remove_file(self.config.todo_file.clone());
        std::fs::rename(bak_path, self.config.todo_file.clone());
    }

    pub fn done(&mut self, line_num: u64) {
        let reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(&self.config.todo_file)
                .unwrap(),
        );
        for (idx, line) in reader.lines().enumerate() {
            if (idx + 1) as u64 == line_num {
                match line {
                    Ok(line) => {
                        println!("todo content: {}", line);
                        self.add(&line, false);
                        self.del((idx + 1) as u64);
                    }
                    Err(e) => {
                        error!("[del cmd] error: {:?}", e);
                    }
                }
            }
        }
    }
}

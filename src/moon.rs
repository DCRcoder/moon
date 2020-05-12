use crate::cfg::Config;
use crate::consts::{CREATED_AT, DEFAULT_TODO_PRI_LEVEL, TODO_MESSAGE, TODO_PRI};
use crate::error::Result;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
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
    pub line_count: u64,
    index_map: BTreeMap<u64, u64>,
}

impl Moon {
    pub fn new(cfg: Config) -> Result<Moon> {
        let writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&cfg.todo_file)?,
        )?;
        let reader = BufReader::new(OpenOptions::new()
                .read(true)
                .open(&cfg.todo_file)?
        );
        let mut line_count: u64 = 0;
        let index_map = BTreeMap::new();
        for (idx, line) in reader.lines().enumerate() {
            match line {
                Ok(_) => {
                    line_count += 1;
                }
                Err(e) => {
                    error!("read error: {:?}", e)
                }
            }
        };
        return Ok(Moon {
            todo_writer: writer,
            line_count: line_count,
            config: cfg,
            index_map: index_map
        });
    }

    pub fn add(&mut self, todo: &str) -> io::Result<usize> {
        let local: DateTime<Local> = Local::now();
        let now = local.format("%Y-%m-%d %H:%M:%S").to_string();
        let t = format!(
            "{}:{}|{}:{}|{}:{}\n",
            TODO_PRI, DEFAULT_TODO_PRI_LEVEL, TODO_MESSAGE, todo, CREATED_AT, now
        );
        return self.todo_writer.write(t.as_bytes());
    }

    pub fn list(&mut self) {
        let reader =  BufReader::new(OpenOptions::new()
        .read(true)
        .open(&self.config.todo_file).unwrap()
        );
        for (idx, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    println!("{} todo_content: {}", idx + 1, line)
                }
                Err(e) => {
                    error!("read error: {:?}", e)
                }
            }
        };
    }
}

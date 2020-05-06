use crate::cfg::Config;
use crate::error::Result;
use crate::consts::{TODO_MESSAGE, CREATED_AT, TODO_PRI, DEFAULT_TODO_PRI_LEVEL};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
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
}

impl Moon {
    pub fn new(cfg :Config) -> Result<Moon> {
        let writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&cfg.todo_file)?,
        )?;
        return Ok(Moon{
            config: cfg,
            todo_writer: writer,
        })
    }

    pub fn add(&mut self, todo: &str) -> io::Result<usize> {
        let local: DateTime<Local> = Local::now();
        let now = local.format("%Y-%m-%d %H:%M:%S").to_string();
        let t = format!("{}:{}|{}:{}|{}:{}\n", TODO_PRI, DEFAULT_TODO_PRI_LEVEL, TODO_MESSAGE, todo, CREATED_AT, now);
        return self.todo_writer.write(t.as_bytes());
    }
}

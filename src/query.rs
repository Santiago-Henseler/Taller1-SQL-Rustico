use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum TypeError {
    InvalidaTable,
    InvalidSintax,
    InvalidColumn,
    Error,
}

pub trait Query {
    fn operate(&mut self, column_index: &str, line: &str) -> Result<String, TypeError>;
}

pub fn mod_file(path: String, instance: &mut dyn Query) -> Result<(), TypeError> {
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|_| TypeError::InvalidaTable)?;
    let mut reader = BufReader::new(file);

    let mut column_index = String::new();
    reader
        .read_line(&mut column_index)
        .map_err(|_| TypeError::Error)?;

    column_index = column_index.replace("\n", "");

    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tmp.csv")
        .map_err(|_| TypeError::Error)?;
    let mut cambio = false;

    writeln!(temp_file, "{}", column_index).map_err(|_| TypeError::Error)?;
    for line in reader.lines() {
        let line_act = line.map_err(|_| TypeError::Error)?;
        let new_line: String = instance.operate(&column_index, line_act.as_str())?;

        if !new_line.is_empty() {
            writeln!(temp_file, "{}", new_line).map_err(|_| TypeError::Error)?;
            cambio = true;
        }
    }

    if cambio {
        fs::copy("tmp.csv", path).map_err(|_| TypeError::Error)?;
    }
    fs::remove_file("tmp.csv").map_err(|_| TypeError::Error)?;

    Ok(())
}

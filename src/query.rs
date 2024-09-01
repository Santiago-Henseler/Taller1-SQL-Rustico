use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;

#[derive(Debug)]
pub enum TypeError{
    InvalidaTable,
    InvalidSintax,
    InvalidColumn,
    FileError,
    Error
}

pub trait Query{
    fn operate(&mut self, column_index: &String, line: String)->String;
}

pub fn mod_file(path:String, instance: &mut dyn Query)-> Result<(), TypeError>{
    let file =  OpenOptions::new().read(true).open(&path).map_err(|_|  TypeError::InvalidaTable)?;
    let mut reader = BufReader::new(file);

    let mut column_index = String::new();
    reader.read_line(&mut column_index).map_err(|_|  TypeError::FileError)?;

    column_index = column_index.replace("\n", "");

    let mut temp_file = OpenOptions::new().write(true).create(true).open("tmp.csv").map_err(|_|  TypeError::FileError)?;
    let mut cambio = false;

    writeln!(temp_file, "{}", column_index).map_err(|_|  TypeError::FileError)?;
    for line in reader.lines() {
        let line = line.map_err(|_| TypeError::FileError)?;
        let new_line: String = instance.operate(&column_index, line);
        
        if new_line != ""{
            writeln!(temp_file, "{}", new_line).map_err(|_|  TypeError::FileError)?;
            cambio = true;
        }
    }

    /* 

    if cambio{
        fs::copy("tmp.csv", path).map_err(|_| TypeError::Error)?;
    }
    fs::remove_file("tmp.csv").map_err(|_|  TypeError::Error)?;
*/
    Ok(())
}
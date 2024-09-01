use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;

use crate::delet;
use delet::Delet;

#[derive(Debug)]
pub enum TypeError{
    InvalidaTable,
    InvalidSintax,
    InvalidColumn,
    Error
}

pub trait Query{
    fn operate(&mut self, column_index: &String, line: String)->String;
}

pub fn parser_kv(str:&str) -> Vec<String>{
    let mut word: String = String::new();

    // cambiar a un replace :)
    str.chars().for_each(|c: char| if c !=')' && c != '(' {word.push(c)});
    word.split(',').map(|s: &str| s.to_string().replace(" ", "")).collect::<Vec<String>>() 
}

pub fn mod_file(path:String, instance: &mut Box<dyn Query>)-> Result<(), TypeError>{
    let file =  OpenOptions::new().read(true).open(&path).map_err(|_|  TypeError::InvalidaTable)?;
    let mut reader = BufReader::new(file);

    let mut column_index = String::new();
    reader.read_line(&mut column_index).map_err(|_|  TypeError::Error)?;

    column_index = column_index.replace("\n", "");

    let mut temp_file = OpenOptions::new().write(true).create(true).open("tmp.csv").map_err(|_|  TypeError::Error)?;

    writeln!(temp_file, "{}", column_index).map_err(|_|  TypeError::Error)?;
    for line in reader.lines() {
        let line = line.map_err(|_| TypeError::Error)?;
        let new_line: String = instance.operate(&column_index, line);
        
        if new_line != ""{
            writeln!(temp_file, "{}", new_line).map_err(|_|  TypeError::Error)?;
        }
    }

   // fs::copy("tmp.csv", path).map_err(|_| TypeError::Error)?;
   // fs::remove_file("tmp.csv").map_err(|_|  TypeError::Error)?;

    Ok(())
}
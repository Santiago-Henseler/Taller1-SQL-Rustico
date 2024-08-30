use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;

use crate::query;
use query::Query;
use query::TypeError;
use query::parser_kv;

#[derive(Debug)]
pub struct Insert{
    values: HashMap<String, String>
}

impl Insert{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();
        let mut hash: HashMap<String, String> = HashMap::new();

        let mut keys: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        for (i, s) in str[1].split("VALUES").enumerate(){
            match i {
                0 => keys = parser_kv(s),
                1 => values = parser_kv(s),
                _ => (),
            }
        }

        for i in 0..keys.len(){
            hash.insert(keys[i].to_owned(), values[i].to_owned());
        }
 
        Self {
            values: hash,
        }
    }

}

impl Query for Insert{
    fn operate(&self, index:&String, _actual:String) -> String{
        let mut word: String = String::new();

        for s in index.replace("\n", "").split(","){
            if self.values.contains_key(s){
                word.push_str(self.values.get(s).unwrap_or(&"".to_string()));
            }else{
                word.push_str(&"NONE".to_string());
            }
            word.push(',');
        }
        word.pop();

        word
    }
}

pub fn insert_reg(path: String, instance: Insert)-> Result<(), TypeError>{

    let mut file = OpenOptions::new().read(true).append(true).open(&path).map_err(|_|  TypeError::InvalidaTable)?;

    let mut reader = BufReader::new(&file);

    let mut column_index = String::new();
    reader.read_line(&mut column_index).map_err(|_| TypeError::Error)?;

    writeln!(file, "{}", instance.operate(&column_index, "".to_string())).map_err(|_| TypeError::Error)?;
    Ok(())
}

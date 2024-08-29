use std::collections::HashMap;

use crate::query;
use query::Query;
use query::TypeError;
use query::parser_kv;

#[derive(Debug)]
pub struct Insert{
    table:String,
    values: HashMap<String, String>
}

impl Insert{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();
        let mut hash: HashMap<String, String> = HashMap::new();
        
        //optimizar esto -_-
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
            hash.insert(keys[i].clone(), values[i].clone());
        }
 
        Self {
            table: table,
            values: hash,
        }
    }

    pub fn get_table(&self) -> &String{
        &self.table
    }


}

impl Query for Insert{
    fn operate(&self, index:String, _actual:String) -> String{
        let mut word: String = String::new();

        for s in index.split(","){
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

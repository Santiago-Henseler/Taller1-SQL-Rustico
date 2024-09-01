use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;

use crate::query;
use query::Query;
use query::TypeError;

#[derive(Debug)]
pub struct Insert{
    values: HashMap<String, String>
}

impl Insert{
    pub fn new(table:String, query: &String) -> Result<Self, TypeError>{
       
        if !query.contains("VALUES"){
            return Err(TypeError::InvalidSintax)
        }
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();
        let mut hash: HashMap<String, String> = HashMap::new();

        make_kv(&mut hash, str)?;

        if hash.is_empty(){
            return Err(TypeError::InvalidSintax)
        }

        Ok(Self {
            values: hash,
        })
    }

}

fn make_kv(hash: &mut HashMap<String,String>, str: Vec<&str>) -> Result<(), TypeError>{

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (i, s) in str[1].split("VALUES").enumerate(){
        match i {
            0 => keys = s.replace('(', "").replace(')', "").split(',').map(|s: &str| s.to_string().replace(" ", "").replace("'","")).collect::<Vec<String>>(),
            1 => values = s.replace('(', "").replace(')', "").split(',').map(|s: &str| s.to_string().replace(" ", "").replace("'","")).collect::<Vec<String>>(),
            _ => (),
        }
    }

    if values[0] == "" || keys.len() < 2{
        return Err(TypeError::InvalidSintax)
    }

    for i in 0..keys.len(){
        hash.insert(keys[i].to_owned(), values[i].to_owned());
    }

    Ok(())
}

impl Query for Insert{
    fn operate(&mut self, index:&String, _actual:String) -> String{
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

pub fn insert_reg(path: String, instance: &mut Insert)-> Result<(), TypeError>{

    let mut file = OpenOptions::new().read(true).append(true).open(&path).map_err(|_|  TypeError::InvalidaTable)?;
    let mut reader = BufReader::new(&file);

    let mut column_index = String::new();
    reader.read_line(&mut column_index).map_err(|_| TypeError::FileError)?;

    writeln!(file, "{}", instance.operate(&column_index, "".to_string())).map_err(|_| TypeError::FileError)?;
    Ok(())
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad) VALUES ");
    let try1: Result<Insert, TypeError>  = Insert::new("tabla1".to_string(), &str1);
    
    match try1{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test2(){
    let str2 = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad)  (id, id_cliente, producto, cantidad)");
    let try2: Result<Insert, TypeError>  = Insert::new("tabla1".to_string(), &str2);
    
    match try2{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test3(){
    let str3 = String::from("INSERT INTO tabla1 VALUES (id, id_cliente, producto, cantidad)");
    let try3: Result<Insert, TypeError>  = Insert::new("tabla1".to_string(), &str3);
    
    match try3{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn operate_test1(){
    let str = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad) VALUES (id, id_cliente, producto, cantidad)");
    let mut instance:Insert  = Insert::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "".to_string());

    assert_eq!(word, "id,id_cliente,producto,cantidad".to_string());
}

#[test]
fn operate_test2(){
    let str = String::from("INSERT INTO tabla1 (id, id_cliente) VALUES (id, id_cliente)");
    let mut instance:Insert  = Insert::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "".to_string());

    assert_eq!(word, "id,id_cliente,NONE,NONE".to_string());
}

//crear este test
#[test]
fn insert_reg_test(){
    assert!(true);
}
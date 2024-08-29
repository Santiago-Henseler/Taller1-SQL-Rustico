
#[derive(Debug)]
pub enum TypeError{
    InvalidaTable,
    InvalidSintax,
    InvalidColumn,
}

pub trait Query{
    fn operate(&self, column_index:String, line:String)->String;
}

pub fn parser_kv(str:&str) -> Vec<String>{
    let mut word: String = String::new();

    str.chars().for_each(|c: char| if c !=')' && c != '(' {word.push(c)});
    word.split(',').map(|s: &str| s.to_string().replace(" ", "")).collect::<Vec<String>>() 
}
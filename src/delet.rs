
use crate::query;
use query::Query;
use query::TypeError;
use query::parser_kv;

#[derive(Debug)]
pub struct Delet{
    table:String,
    conditions: Vec<String>,
}

impl Delet{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        // tratar de optimizar
        let conditions: Vec<String> = str[1].split("WHERE").collect::<Vec<&str>>()[1].split("=").map(|s: &str| s.to_string().replace(" ", "").replace("'", "")).collect::<Vec<String>>();

        Self {
            table: table,
            conditions: conditions,
        }
    }

    pub fn get_table(&self) -> &String{
        &self.table
    }
}

impl Query for Delet{
    fn operate(&self, index:String, actual:String) -> String{
        let index_vec: Vec<String> = index.split(",").map(|f| f.to_string()).collect::<Vec<String>>();
        let actual_vec: Vec<String> = actual.split(",").map(|f| f.to_string()).collect::<Vec<String>>();

        if !index_vec.contains(&self.conditions[0]){
            println!("error"); //hacer algo con el error
        }

        let mut condition = false;
        for (i, s) in index_vec.into_iter().enumerate(){
            if s == self.conditions[0] && actual_vec[i] == self.conditions[1]{
                condition = true;
            }
        }
        
        match condition{
            true => "".to_string(),
            false => actual,
        }
    }
}

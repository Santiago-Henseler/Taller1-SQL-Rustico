use crate::query;
use query::Query;
use query::TypeError;
use query::parser_kv;

#[derive(Debug)]
pub struct Update{
    table:String,
   // values: Vec<String>,
    conditions: Vec<String>,
}

impl Update{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        // tratar de optimizar
        let conditions: Vec<String> = str[1].split("WHERE").collect::<Vec<&str>>()[1].split("=").map(|s: &str| s.to_string().replace(" ", "").replace("'", "")).collect::<Vec<String>>();

        println!("{:?}", query.split("SET").collect::<Vec<&str>>());

        Self {
            table: table,
            conditions: conditions,
        }
    }

    pub fn get_table(&self) -> &String{
        &self.table
    }
}

impl Query for Update{
    fn operate(&self, index:String, actual:String) -> String{
        let index_vec: Vec<String> = index.split(",").map(|f| f.to_string()).collect::<Vec<String>>();
        let actual_vec: Vec<String> = actual.split(",").map(|f| f.to_string()).collect::<Vec<String>>();


        
        match true{
            true => "".to_string(),
            false => actual,
        }
    }
}

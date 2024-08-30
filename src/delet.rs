use crate::query;
use query::Query;
use query::TypeError;
use crate::condition;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;

#[derive(Debug)]
pub struct Delet{
    conditions: Expresion,
}

impl Delet{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        Self {
            conditions: get_conditions(str[1].split("WHERE").collect::<Vec<&str>>()[1].replace(',', " AND ").as_str()),
        }
    }
}

impl Query for Delet{
    fn operate(&self, index:&String, actual:String) -> String{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", "")),
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", "")) && evaluar(c_der, index, &actual.replace("\n", "")),
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", "")),
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", "")) || evaluar(c_der, index, &actual.replace("\n", "")),
        };

        match condition{
            true => "".to_string(),
            false => actual,
        }
    }
}

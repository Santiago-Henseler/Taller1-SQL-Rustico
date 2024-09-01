use std::collections::HashMap;

use crate::query;
use query::Query;
use query::TypeError;
use crate::condition;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;
use crate::sortCondition;
use sortCondition::SortExpresion;
use sortCondition::make_sort_condition;

#[derive(Debug)]
pub struct Select{
    conditions: Expresion,
    sort: bool,
    sort_conditions: SortExpresion,
    lines: HashMap<String, String>
}

impl Select{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>()[1].split("WHERE").collect::<Vec<&str>>()[1].split("ORDER BY").collect::<Vec<&str>>();
        let mut sort = false;
        let mut sort_expresion = SortExpresion::None;
        let mut hash: HashMap<String, String> = HashMap::new();

        if str.len() > 1{
            sort = true;
            sort_expresion = make_sort_condition(str[1]);
        }

        Self {
            conditions: get_conditions(str[0].replace(',', " AND ").as_str()),
            sort: sort,
            sort_conditions: sort_expresion,
            lines: hash,
        }
    }
}

impl Query for Select{
    fn operate(&mut self, index:&String, actual:String) -> String{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", "")),
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", "")) && evaluar(c_der, index, &actual.replace("\n", "")),
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", "")),
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", "")) || evaluar(c_der, index, &actual.replace("\n", "")),
        };

        match condition{
            true => {
                if self.sort{
                    let vec = actual.split(',').collect::<Vec<&str>>();
                    let key = match &self.sort_conditions {
                        SortExpresion::SortCondition(c) =>{
                            if let Some(pos) = index.split(',').collect::<Vec<&str>>().iter().position(|&x| x == &c.column_index){
                                vec[pos].to_string()
                            }else{
                                "".to_string()
                            }
                        },
                        SortExpresion::None => "".to_string(),
                    };
                    self.lines.insert(key, actual);
                }else{
                    println!("{}", actual);
                }
            },
            false => (),
        }

        "".to_string()
    }
}

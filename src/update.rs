use crate::query;
use crate::condition;
use query::Query;
use query::TypeError;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;

#[derive(Debug)]
pub struct Update{
    conditions: Expresion,
    set: Expresion,
}

impl Update{
    pub fn new(table:String, query: &String) -> Self{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>()[1].split("WHERE").collect::<Vec<&str>>();

         // Si no tiene WHERE tirar sintax error

            // funciona?

          // Si no tiene SET tirar sintax error

        Self {
            conditions: get_conditions(str[1].replace(',', " AND ").as_str()),
            set: get_conditions(str[0].replace(',', " AND ").split("SET").collect::<Vec<&str>>()[1])
        }
    }
}

fn update_str(expresion: &Expresion, index: &String, actual: String) -> String{

    let mut new_string: String = String::from("");

    match expresion{
        Expresion::Condicion(c) => {
            let act_vec = actual.split(",").collect::<Vec<&str>>();
            for (i, s) in index.replace("\n", "").split(",").enumerate(){
                if s.to_string() == c.column_index{
                    new_string.push_str(&c.value);
                }else{
                    new_string.push_str(act_vec[i]);
                }
                new_string.push(',');
            }
            new_string.pop();
            },
        _ => print!("error"),
    }

    new_string
}

impl Query for Update{
    fn operate(&mut self, index:&String, actual:String) -> String{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", "")),
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", "")) && evaluar(c_der, index, &actual.replace("\n", "")),
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", "")),
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", "")) || evaluar(c_der, index, &actual.replace("\n", "")),
        };

        match condition{
            true => update_str(&self.set, index, actual),
            false => actual,
        }
    }
}

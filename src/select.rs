use crate::query;
use crate::sort_condition::sort;
use query::Query;
use query::TypeError;
use crate::condition;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;
use crate::sort_condition;
use sort_condition::SortExpresion;
use sort_condition::make_sort_condition;

#[derive(Debug)]
pub struct Select{
    conditions: Expresion,
    sort: bool,
    sort_conditions: SortExpresion,
    lines: Vec<String>
}

impl Select{
    pub fn new(table:String, query: &String) -> Result<Self, TypeError>{
        
        if !query.contains("WHERE"){
            return Err(TypeError::InvalidSintax)
        }

        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>()[1].split("WHERE").collect::<Vec<&str>>()[1].split("ORDER BY").collect::<Vec<&str>>();
        let mut sort = false;
        let mut sort_expresion: SortExpresion = SortExpresion::None;
        let mut lines: Vec<String> = Vec::new();

        if str.len() > 1{
            sort = true;
            sort_expresion = make_sort_condition(str[1])?;
        }

        let condition = get_conditions(str[0].replace(',', " AND ").as_str())?;

        Ok(Self {
            conditions: condition,
            sort: sort,
            sort_conditions: sort_expresion,
            lines: lines,
        })
    }

    pub fn print(&mut self) -> Result<(), TypeError>{
        if !self.sort {
            return Ok(())
        }
        if self.lines.len() < 1{
            return  Ok(())
        }
        sort(&mut self.lines, &self.sort_conditions)?;
        for i in self.lines.iter(){
            println!("{}", &i);
        }
        Ok(())
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
                    if self.lines.len() == 0{
                        self.lines.push(index.to_owned());
                    }
                    self.lines.push(actual);
                }else{
                    println!("{}", actual);
                }
            },
            false => (),
        }
        "".to_string()
    }
}

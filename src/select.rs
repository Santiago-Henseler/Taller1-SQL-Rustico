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

/// La representación e implementación del comando Select de SQL
#[derive(Debug)]
pub struct Select{
    columns: Vec<String>,
    conditions: Expresion,
    sort: bool,
    sort_conditions: SortExpresion,
    lines: Vec<String>
}

impl Select{
    /// Crea la representación de Select 
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Select o InvalidSintax si la query no es valida
    /// 
    pub fn new(table:String, query: &String) -> Result<Self, TypeError>{
        
        if !query.contains("FROM") || table == ""{ return Err(TypeError::InvalidSintax)};

        let mut str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();
        let mut conditions: Expresion = Expresion::All;
        let mut sort = false;
        let mut sort_conditions: SortExpresion = SortExpresion::None;
        let lines: Vec<String> = Vec::new();
        let mut columns = Vec::new();

        let str_columns = str[0].split("SELECT").collect::<Vec<&str>>()[1].split("FROM").collect::<Vec<&str>>()[0].replace(" ", "");

        if str_columns == ""{ return Err(TypeError::InvalidSintax)};

        if str_columns != "*"{columns = str_columns.split(",").map(|s| s.to_string()).collect::<Vec<String>>();}

        if query.contains("WHERE"){
            str = str[1].split("WHERE").collect::<Vec<&str>>()[1].split("ORDER BY").collect::<Vec<&str>>();
            conditions = get_conditions(str[0].replace(',', " AND ").as_str())?;
            if str.len() > 1{
                sort = true;
                sort_conditions = make_sort_condition(str[1])?;
            }
        }else if query.contains("ORDER BY"){
            sort = true;
            sort_conditions = make_sort_condition(str[1].split("ORDER BY").collect::<Vec<&str>>()[1])?;
        }

        Ok(Self{columns,conditions,sort, sort_conditions,lines,})
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

///
/// Selecciona las columnas de la fila actual y las guarda devuelve en un String.
/// Si no devuelve un InvalidColumn.
/// 
fn select_columns(actual: String, columns: &Vec<String>, index: &String) -> Result<String, TypeError>{

    if columns.len() == 0{
        return Ok(actual)
    }

    let mut new_string: String = String::from("");
    let mut col_select = 0;

    let act_vec = actual.split(",").collect::<Vec<&str>>();
    for (i, col) in index.replace("\n", "").split(",").enumerate(){
        if columns.contains(&col.to_string()){
            new_string.push_str(act_vec[i]);
            new_string.push(',');
            col_select += 1;
        }
    }

    if col_select != columns.len(){
        return Err(TypeError::InvalidColumn)
    }

    new_string.pop();
    Ok(new_string)
}

///
/// La implementación de operate para Select
/// 
/// Si se cumple la condición selecciona la fila y si se quiere ordenar usa select_columns y agrega ese valor al vector de Select.
/// Si no se printea directamente.
/// 
impl Query for Select{
    fn operate(&mut self, index:&String, actual:String) -> Result<String, TypeError>{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", ""))?,
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", ""))? && evaluar(c_der, index, &actual.replace("\n", ""))?,
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", ""))?,
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", ""))? || evaluar(c_der, index, &actual.replace("\n", ""))?,
            Expresion::All => true,
        };

        match condition{
            true => {
                if self.sort{
                    if self.lines.len() == 0{
                        self.lines.push(select_columns(index.clone(), &self.columns, index)?);
                    }
                    self.lines.push(select_columns(actual, &self.columns, index)?);
                }else{
                    println!("{}", actual);
                }
            },
            false => (),
        }
        Ok("".to_string())
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("SELECT id, id_cliente tabla1 WHERE id_cliente > 1 ORDER BY id ASC");
    let try1: Result<Select, TypeError>  = Select::new("tabla1".to_string(), &str1);
    
    match try1{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test2(){
    let str2 = String::from("SELECT FROM tabla1 WHERE id_cliente > 1 ORDER BY id ASC");
    let try2: Result<Select, TypeError>  = Select::new("tabla1".to_string(), &str2);
    
    match try2{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test3(){
    let str3 = String::from("SELECT id FROM");
    let try3: Result<Select, TypeError>  = Select::new("".to_string(), &str3);
    
    match try3{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test4(){
    let str4 = String::from("SELECT id FROM tabla1");
    let try4: Result<Select, TypeError>  = Select::new("tabla1".to_string(), &str4);
    
    match try4{ 
        Err(TypeError::InvalidSintax) => assert!(false),
        _ => assert!(true)
    }
}

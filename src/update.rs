use std::collections::HashMap;

use crate::query;
use crate::condition;
use query::Query;
use query::TypeError;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;


/// La representación e implementación del comando Update de SQL
#[derive(Debug)]
pub struct Update{
    conditions: Expresion,
    set: HashMap<String,String>,
}

/// 
/// Crea un hash con la columna y el valor a actualizar
/// 
/// Si no puede crear el par columna-valor devuelve un error de InvalidSintax
/// 
fn make_kv(hash: &mut HashMap<String,String>, str: &str) -> Result<(), TypeError>{
   
    for s in str.split(','){
        let vecs = s.split('=').collect::<Vec<&str>>();
        if vecs.len() != 2{
            return  Err(TypeError::InvalidSintax)
        }
        hash.insert(vecs[0].to_string().replace(" ", ""), vecs[1].to_string().replace(" ", "").replace("'", ""));
    }

    Ok(())
}

impl Update{
    /// Crea la representación de Update 
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Update o InvalidSintax si la query no es valida
    /// 
    pub fn new(table:String, query: &str) -> Result<Self, TypeError>{

        if !query.contains("WHERE") || !query.contains("SET"){
            return Err(TypeError::InvalidSintax)
        }
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>()[1].split("WHERE").collect::<Vec<&str>>();

        let mut hash: HashMap<String, String> = HashMap::new();
        make_kv(&mut hash, str[0].split("SET").collect::<Vec<&str>>()[1])?;

        let conditions = get_conditions(str[1].replace(',', " AND ").as_str())?;
 
        Ok(Self {
            conditions,
            set: hash
        })
    }
}

///
/// Recibe la expresión y la fila a modificar
/// 
/// Devuelve la fila modificada 
/// 
fn update_str(hash: &HashMap<String,String>, index: &str, actual: String) -> Result<String, TypeError>{

    let mut new_string: String = String::from("");

    let act_vec = actual.split(",").collect::<Vec<&str>>();
    for (i, col) in index.replace("\n", "").split(",").enumerate(){
        if hash.contains_key(&col.to_string()){
            new_string.push_str(hash.get(&col.to_string()).unwrap_or(&"".to_string()));
        }else{
            new_string.push_str(act_vec[i]);
        }
        new_string.push(',');
    }
    new_string.pop();

    Ok(new_string)
}

///
/// La implementación de operate para Update
/// 
/// Si se cumple la condición para actualizar la fila usa update_str y devuelve la fila cambiada.
/// Si no devuelve la fila sin modificarla
/// 
impl Query for Update{
    fn operate(&mut self, index:&str, actual:String) -> Result<String, TypeError>{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", ""))?,
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", ""))? && evaluar(c_der, index, &actual.replace("\n", ""))?,
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", ""))?,
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", ""))? || evaluar(c_der, index, &actual.replace("\n", ""))?,
            Expresion::All => true,
        };

        match condition{
            true => update_str(&self.set, index, actual),
            false => Ok(actual),
        }
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("UPDATE tabla1 id = 99 WHERE id_cliente = 1 ");
    let try1: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str1);
    
    match try1{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test2(){
    let str2 = String::from("UPDATE tabla1 SET WHERE id_cliente = 1");
    let try2: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str2);
    
    match try2{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test3(){
    let str3 = String::from("UPDATE tabla1 SET id = 99  id_cliente = 1");
    let try3: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str3);
    
    match try3{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test4(){
    let str4 = String::from("UPDATE tabla1 SET id = 99 WHERE dadadad");
    let try4: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str4);
    
    match try4{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn operate_test1(){
    let str = String::from("UPDATE tabla1 SET id = 99, id_cliente = 10 WHERE id_cliente = 1");
    let mut instance:Update  = Update::new("tabla1".to_string(), &str).unwrap();

    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1".to_string()).unwrap();

    assert_eq!(word, "99,10,Laptop,1".to_string());
}

#[test]
fn operate_test2(){
    let str = String::from("UPDATE tabla1 SET id = 99 WHERE id_cliente = 3");
    let mut instance:Update  = Update::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1".to_string()).unwrap();

    assert_eq!(word, "101,1,Laptop,1".to_string());
}
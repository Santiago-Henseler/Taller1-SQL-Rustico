use std::collections::HashMap;

use crate::query;
use crate::condition;
use query::Query;
use query::TypeError;
use condition::obtener_condicion;
use condition::Expresion;
use condition::evaluar_condicion;


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
fn hacer_kv(hash: &mut HashMap<String,String>, str: &str) -> Result<(), TypeError>{
   
    for s in str.split(','){
        let vecs = s.split('=').collect::<Vec<&str>>();
        if vecs.len() != 2{
            return  Err(TypeError::InvalidSintax)
        }
        hash.insert(vecs[0].to_string().replace(" ", ""), vecs[1].to_string().replace(" ", "").replace("'", ""));
    }

    Ok(())
}

fn hacer_set(setstr: &str ) -> Result<HashMap<String, String>, TypeError>{

    let mut kv_set: HashMap<String, String> = HashMap::new();

    if let Some(set_values) = setstr.split("SET").collect::<Vec<&str>>().get(1){
        hacer_kv(&mut kv_set, set_values)?;
    }else{
        return Err(TypeError::InvalidSintax)
    }

    Ok(kv_set)
}

impl Update{
    /// Crea la representación de Update 
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Update o InvalidSintax si la query no es valida
    /// 
    pub fn new(table:String, query: &str) -> Result<Self, TypeError>{

        if !query.contains(" SET "){
            return Err(TypeError::InvalidSintax)
        }

        let query_vec: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        if let Some(q) = query_vec.get(1){
            
            if q.contains(" WHERE "){
                let query_slice = q.split("WHERE").collect::<Vec<&str>>();

                if query_slice.len() != 2 {
                    return Err(TypeError::InvalidSintax)
                }

                return Ok(Self {
                    conditions: obtener_condicion(query_slice[1].replace(',', " AND ").as_str())?,
                    set: hacer_set(query_slice[0])?
                })
            }

            Ok(Self {
                conditions: Expresion::All,
                set: hacer_set(q)?
            })
        }else{
            return Err(TypeError::InvalidSintax)
        }
    }
}

///
/// Recibe la expresión y la fila a modificar
/// 
/// Devuelve la fila modificada 
/// 
fn update_str(hash: &HashMap<String,String>, index: &str, actual: &str) -> Result<String, TypeError>{

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
    fn operate(&mut self, index:&str, actual:&str) -> Result<String, TypeError>{

        let condition: bool = evaluar_condicion(&self.conditions, index, &actual)?;

        match condition{
            true => update_str(&self.set, index, actual),
            false => Ok(actual.to_string()),
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

    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1").unwrap();

    assert_eq!(word, "99,10,Laptop,1".to_string());
}

#[test]
fn operate_test2(){
    let str = String::from("UPDATE tabla1 SET id = 99 WHERE id_cliente = 3");
    let mut instance:Update  = Update::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1").unwrap();

    assert_eq!(word, "101,1,Laptop,1".to_string());
}
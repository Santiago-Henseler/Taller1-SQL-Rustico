use crate::condition;
use crate::query;
use condition::evaluar_condicion;
use condition::obtener_condicion;
use condition::Expresion;
use query::Query;
use query::TypeError;

/// La representación e implementación del comando Delete de SQL
#[derive(Debug)]
pub struct Delete {
    conditions: Expresion,
}

impl Delete {
    /// Crea la representación de Delete
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Delete o InvalidSintax si la query no es valida
    ///
    pub fn new(table: String, query: &str) -> Result<Self, TypeError> {
        let query_vec: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        if !query.contains(" WHERE ") {
            return Ok(Self {
                conditions: Expresion::All,
            });
        }

        if let Some(q) = query_vec.get(1) {
            let query_slice = q.split("WHERE").collect::<Vec<&str>>();

            if let Some(cond) = query_slice.get(1) {
                return Ok(Self {
                    conditions: obtener_condicion(cond.replace(',', " AND ").as_str())?,
                });
            }
        }

        Err(TypeError::InvalidSintax)
    }
}

///
/// La implementación de operate para Delete
///
/// Si se cumple la condición para borrar la fila devuelve un string vacio.
/// Si no devuelve la fila sin modificarla
///
impl Query for Delete {
    fn operate(&mut self, index: &str, actual: &str) -> Result<String, TypeError> {
        let condition: bool = evaluar_condicion(&self.conditions, index, actual)?;

        match condition {
            true => Ok("".to_string()),
            false => Ok(actual.to_string()),
        }
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("DELET FROM tabla1 id_cliente = 1 ");
    let try1: Result<Delete, TypeError> = Delete::new("tabla1".to_string(), &str1);

    match try1 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test2() {
    let str2 = String::from("DELET FROM tabla1 WHERE  ");
    let try2: Result<Delete, TypeError> = Delete::new("tabla1".to_string(), &str2);

    match try2 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test3() {
    let str3 = String::from("DELET FROM tabla1 WHERE = 1 ");
    let try3: Result<Delete, TypeError> = Delete::new("tabla1".to_string(), &str3);

    match try3 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn operate_test1() {
    let str = String::from("DELETe FROM tabla1 WHERE id_cliente = 1");
    let mut instance: Delete = Delete::new("tabla1".to_string(), &str).unwrap();

    let word = instance
        .operate(
            &"id,id_cliente,producto,cantidad".to_string(),
            "101,1,Laptop,1",
        )
        .unwrap();

    assert_eq!(word, "".to_string());
}

#[test]
fn operate_test2() {
    let str = String::from("Delete FROM tabla1 WHERE id_cliente = 3 AND id = 99");
    let mut instance: Delete = Delete::new("tabla1".to_string(), &str).unwrap();

    let word = instance
        .operate(
            &"id,id_cliente,producto,cantidad".to_string(),
            "101,1,Laptop,1",
        )
        .unwrap();

    assert_eq!(word, "101,1,Laptop,1".to_string());
}

#[test]
fn operate_test3() {
    let str = String::from("Delete FROM tabla1 WHERE id_cliente = 1 AND (NOT id = 99)");
    let mut instance: Delete = Delete::new("tabla1".to_string(), &str).unwrap();

    let word = instance
        .operate(
            &"id,id_cliente,producto,cantidad".to_string(),
            "101,1,Laptop,1",
        )
        .unwrap();

    assert_eq!(word, "".to_string());
}

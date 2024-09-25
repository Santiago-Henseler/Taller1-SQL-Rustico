use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use crate::query;
use query::Query;
use query::TypeError;

/// La representación e implementación del comando Insert de SQL
#[derive(Debug)]
pub struct Insert {
    values: HashMap<String, String>,
}

impl Insert {
    /// Crea la representación de Insert
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Insert o InvalidSintax si la query no es valida
    ///
    pub fn new(table: String, query: &str) -> Result<Self, TypeError> {
        if !query.contains(" VALUES ") || !query.contains(" INTO ") {
            return Err(TypeError::InvalidSintax);
        }

        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();
        let mut kv_to_insert: HashMap<String, String> = HashMap::new();
        let s = str[1].split("VALUES").collect::<Vec<&str>>();

        hacer_kv(&mut kv_to_insert, &s)?;

        if kv_to_insert.is_empty() {
            return Err(TypeError::InvalidSintax);
        }

        Ok(Self {
            values: kv_to_insert,
        })
    }
}

fn limpiar_valor(str: &str) -> Vec<String> {
    str.replace(['(', ')'], "")
        .split(',')
        .map(|s: &str| s.to_string().replace(" ", "").replace("'", ""))
        .collect::<Vec<String>>()
}

///
/// Crea un hash con la columna y el valor a insertar
///
/// Si no puede crear el par columna-valor devuelve un error de InvalidSintax
///
fn hacer_kv(hash: &mut HashMap<String, String>, str: &[&str]) -> Result<(), TypeError> {
    match (str.get(0), str.get(1)) {
        (Some(keys), Some(values)) => {
            let vec_keys: Vec<String> = limpiar_valor(keys);
            let vec_values: Vec<String> = limpiar_valor(values);

            if vec_values.is_empty() {
                return Err(TypeError::InvalidSintax);
            }

            if vec_keys.len() < 1 || vec_keys.len() != vec_values.len() {
                return Err(TypeError::InvalidSintax);
            }

            for i in 0..vec_keys.len() {
                hash.insert(vec_keys[i].to_string(), vec_values[i].to_string());
            }

            return Ok(());
        }
        _ => return Err(TypeError::InvalidSintax),
    }
}

///
/// La implementación de operate para Insert
///
/// Crea la nueva fila a insertar. Donde no encuentra valores para una columna inserta NONE
/// Y si alguna columna no existe devuelve InvalidColumn
///
impl Query for Insert {
    fn operate(&mut self, index: &str, _actual: &str) -> Result<String, TypeError> {
        let mut word: String = String::new();
        let mut i = 0;

        for s in index.replace("\n", "").split(",") {
            if self.values.contains_key(s) {
                word.push_str(self.values.get(s).unwrap_or(&"".to_string()));
                i += 1;
            } else {
                word.push_str("NONE");
            }
            word.push(',');
        }
        word.pop();

        if self.values.len() != i {
            return Err(TypeError::InvalidColumn);
        }

        Ok(word)
    }
}

///
/// Inserta al final de la tabla la nueva fila generada por operate
///
/// Si no logra insertarla devuelve el tipo de error correspondiente
///
pub fn agregar_reg(path: String, instance: &mut Insert) -> Result<(), TypeError> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(&path)
        .map_err(|_| TypeError::InvalidaTable)?;
    let mut reader = BufReader::new(&file);

    let mut column_index = String::new();
    reader
        .read_line(&mut column_index)
        .map_err(|_| TypeError::Error)?;

    file.seek(SeekFrom::End(-1)).map_err(|_| TypeError::Error)?;

    let mut ultima = [0; 1];
    file.read_exact(&mut ultima).map_err(|_| TypeError::Error)?;

    if ultima != [b'\n'] {
        writeln!(file).map_err(|_| TypeError::Error)?;
    }

    writeln!(file, "{}", instance.operate(&column_index, "")?).map_err(|_| TypeError::Error)?;
    Ok(())
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad) VALUES ");
    let try1: Result<Insert, TypeError> = Insert::new("tabla1".to_string(), &str1);

    match try1 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test2() {
    let str2 = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad)  (id, id_cliente, producto, cantidad)");
    let try2: Result<Insert, TypeError> = Insert::new("tabla1".to_string(), &str2);

    match try2 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test3() {
    let str3 = String::from("INSERT INTO tabla1 VALUES (id, id_cliente, producto, cantidad)");
    let try3: Result<Insert, TypeError> = Insert::new("tabla1".to_string(), &str3);

    match try3 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn operate_test1() {
    let str = String::from("INSERT INTO tabla1 (id, id_cliente, producto, cantidad) VALUES (id, id_cliente, producto, cantidad)");
    let mut instance: Insert = Insert::new("tabla1".to_string(), &str).unwrap();

    let word = instance
        .operate(&"id,id_cliente,producto,cantidad".to_string(), "")
        .unwrap();

    assert_eq!(word, "id,id_cliente,producto,cantidad".to_string());
}

#[test]
fn operate_test2() {
    let str = String::from("INSERT INTO tabla1 (id, id_cliente) VALUES (id, id_cliente)");
    let mut instance: Insert = Insert::new("tabla1".to_string(), &str).unwrap();

    let word = instance
        .operate(&"id,id_cliente,producto,cantidad".to_string(), "")
        .unwrap();

    assert_eq!(word, "id,id_cliente,NONE,NONE".to_string());
}

#[test]
fn operate_test3() {
    let str = String::from("INSERT INTO tabla1 (id, calves) VALUES (id, id_cliente)");
    let mut instance: Insert = Insert::new("tabla1".to_string(), &str).unwrap();

    let word: Result<String, TypeError> =
        instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "");

    match word {
        Err(TypeError::InvalidColumn) => assert!(true),
        _ => assert!(false),
    }
}

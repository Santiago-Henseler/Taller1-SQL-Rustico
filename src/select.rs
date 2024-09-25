use crate::condition;
use crate::query;
use crate::sort_condition;
use crate::sort_condition::sort;
use condition::evaluar_condicion;
use condition::obtener_condicion;
use condition::Expresion;
use query::Query;
use query::TypeError;
use sort_condition::hacer_expresion_sort;
use sort_condition::SortExpresion;

/// La representación e implementación del comando Select de SQL
#[derive(Debug)]
pub struct Select {
    columns: Vec<String>,
    conditions: Expresion,
    sort_expresion: SortExpresion,
    lines: Vec<String>,
}

impl Select {
    /// Crea la representación de Select
    ///
    /// Necesita recibir la tabla con la que va a operar y la query pedida
    ///  
    /// Devuelve Select o InvalidSintax si la query no es valida
    ///
    pub fn new(table: String, query: &str) -> Result<Self, TypeError> {
        if table.is_empty() {
            return Err(TypeError::InvalidaTable);
        };

        let query_vec: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        if !query.contains(" FROM ") || query_vec.is_empty() {
            return Err(TypeError::InvalidSintax);
        };

        let (conditions, sort_expresion) = obtener_sort_cond(query, &query_vec)?;

        Ok(Self {
            columns: obtener_columnas(query_vec[0])?,
            conditions,
            sort_expresion,
            lines: Vec::new(),
        })
    }

    pub fn print(&mut self) -> Result<(), TypeError> {
        if self.lines.is_empty() {
            return Ok(());
        }

        match self.sort_expresion {
            SortExpresion::None => (),
            _ => sort(&mut self.lines, &self.sort_expresion)?,
        }

        let mut index = "";

        for (i, s) in self.lines.iter().enumerate() {
            if i == 0 {
                index = s.as_str();
                continue;
            }
            println!("{}", seleccionar_columns(s, &self.columns, index)?);
        }
        Ok(())
    }
}

fn obtener_columnas(query_vec: &str) -> Result<Vec<String>, TypeError> {
    let mut columns: Vec<String> = Vec::new();

    if let Some(vec_columns) = query_vec.split("SELECT").collect::<Vec<&str>>().get(1) {
        if vec_columns.is_empty() {
            return Err(TypeError::InvalidSintax);
        };

        let str_columns = vec_columns.split("FROM").collect::<Vec<&str>>()[0].replace(" ", "");

        if str_columns.is_empty() {
            return Err(TypeError::InvalidSintax);
        };

        if str_columns != "*" {
            columns = str_columns
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
        }
    }

    Ok(columns)
}

fn obtener_sort_cond(
    query: &str,
    query_vec: &Vec<&str>,
) -> Result<(Expresion, SortExpresion), TypeError> {
    if query_vec.len() != 2 {
        return Err(TypeError::InvalidSintax);
    };

    if query.contains(" WHERE ") && query.contains(" ORDER BY ") {
        if let Some(conds) = query_vec[1].split("WHERE").collect::<Vec<&str>>().get(1) {
            let conds_vec = conds.split("ORDER BY").collect::<Vec<&str>>();

            if conds_vec.len() != 2 {
                return Err(TypeError::InvalidSintax);
            };

            return Ok((
                obtener_condicion(conds_vec[0].replace(',', " AND ").as_str())?,
                hacer_expresion_sort(conds_vec[1])?,
            ));
        }
    } else if query.contains(" WHERE ") {
        let where_vec = query_vec[1].split("WHERE").collect::<Vec<&str>>();

        if where_vec.len() != 2 {
            return Err(TypeError::InvalidSintax);
        };

        return Ok((
            obtener_condicion(where_vec[1].replace(',', " AND ").as_str())?,
            SortExpresion::None,
        ));
    } else if query.contains(" ORDER BY ") {
        return Ok((
            Expresion::All,
            hacer_expresion_sort(query_vec[1].split("ORDER BY").collect::<Vec<&str>>()[1])?,
        ));
    } else {
        return Ok((Expresion::All, SortExpresion::None));
    }

    Err(TypeError::InvalidSintax)
}

///
/// Selecciona las columnas de la fila actual y las guarda devuelve en un String.
/// Si no devuelve un InvalidColumn.
///
fn seleccionar_columns(actual: &str, columns: &[String], index: &str) -> Result<String, TypeError> {
    if columns.is_empty() {
        return Ok(actual.to_string());
    }

    let mut new_string: String = String::from("");
    let mut col_select = 0;

    let act_vec = actual.split(",").collect::<Vec<&str>>();

    for col in columns {
        if let Some(i) = index
            .replace("\n", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .position(|&x| x == col)
        {
            new_string.push_str(act_vec[i]);
            new_string.push(',');
            col_select += 1;
        }
    }

    if col_select != columns.len() {
        return Err(TypeError::InvalidColumn);
    }

    new_string.pop();
    Ok(new_string)
}

///
/// La implementación de operate para Select
///
/// Si se cumple la condición selecciona la fila y si se quiere ordenar usa seleccionar_columns y agrega ese valor al vector de Select.
/// Si no se printea directamente.
///
impl Query for Select {
    fn operate(&mut self, index: &str, actual: &str) -> Result<String, TypeError> {
        let condition: bool = evaluar_condicion(&self.conditions, index, &actual)?;

        match condition {
            true => {
                if self.lines.is_empty() {
                    self.lines.push(index.to_string());
                }
                self.lines.push(actual.to_string());
            }
            false => (),
        }

        Ok("".to_string())
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("SELECT id, id_cliente tabla1 WHERE id_cliente > 1 ORDER BY id ASC");
    let try1: Result<Select, TypeError> = Select::new("tabla1".to_string(), &str1);

    match try1 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test2() {
    let str2 = String::from("SELECT FROM tabla1 WHERE id_cliente > 1 ORDER BY id ASC");
    let try2: Result<Select, TypeError> = Select::new("tabla1".to_string(), &str2);

    match try2 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test3() {
    let str3 = String::from("SELECT id FROM");
    let try3: Result<Select, TypeError> = Select::new("".to_string(), &str3);

    match try3 {
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn sintax_error_test4() {
    let str4 = String::from("SELECT id FROM tabla1");
    let try4: Result<Select, TypeError> = Select::new("tabla1".to_string(), &str4);

    match try4 {
        Err(TypeError::InvalidSintax) => assert!(false),
        _ => assert!(true),
    }
}

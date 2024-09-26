use crate::query::TypeError;

/// La representación e implementación de las expresiones de busqueda
#[derive(Debug)]
pub enum Operador {
    Mayor,
    MayorOIgual,
    Menor,
    MenorOIgual,
    Igual,
    Error,
}

#[derive(Debug)]
pub struct Condicion {
    pub operador: Operador,
    pub valores: (String, String),
}

#[derive(Debug)]
pub enum Expresion {
    Condicion(Condicion),
    Not(Box<Expresion>),
    And(Box<Expresion>, Box<Expresion>),
    Or(Box<Expresion>, Box<Expresion>),
    All,
}

fn separar_condition(str: &str) -> (Operador, Vec<&str>) {
    if str.contains(">=") {
        return (
            Operador::MayorOIgual,
            str.split(">=").collect::<Vec<&str>>(),
        );
    } else if str.contains("<=") {
        return (
            Operador::MenorOIgual,
            str.split("<=").collect::<Vec<&str>>(),
        );
    } else if str.contains("<") {
        return (Operador::Menor, str.split("<").collect::<Vec<&str>>());
    } else if str.contains(">") {
        return (Operador::Mayor, str.split(">").collect::<Vec<&str>>());
    } else if str.contains("=") {
        return (Operador::Igual, str.split("=").collect::<Vec<&str>>());
    }
    (Operador::Error, Vec::<&str>::new())
}

///
/// Crea la Condicion de busqueda
/// Si no se puede crear devuelve InvalidSintax.
///
fn hacer_condition(str: &str) -> Result<Condicion, TypeError> {
    let str_limpio: String = str.replace(" ", "");

    let (op, strvec) = separar_condition(&str_limpio);

    if strvec.len() != 2 {
        return Err(TypeError::InvalidSintax);
    }

    match op {
        Operador::Error => Err(TypeError::InvalidSintax),
        op => Ok(Condicion {
            operador: op,
            valores: (strvec[0].replace("'", "").to_string(), strvec[1].replace("'", "").to_string()),
        }),
    }
}

fn exp(exp_vec: Vec<&str>) -> Result<Vec<Expresion>, TypeError> {
    let mut exprs: Vec<Expresion> = Vec::new();
    for part in exp_vec {
        exprs.push(obtener_condicion(part)?);
    }
    Ok(exprs)
}

///
/// Crea la Expresion de busqueda
/// Si no se puede crear devuelve InvalidSintax.
///
pub fn obtener_condicion(condition: &str) -> Result<Expresion, TypeError> {
    let cond_cls = condition.trim();

    if cond_cls.starts_with("(") && cond_cls.ends_with(")") {
        return obtener_condicion(&cond_cls[1..cond_cls.len() - 1]);
    }

    if cond_cls.starts_with("NOT ") {
        let cond_not = cond_cls
            .strip_prefix("NOT ")
            .ok_or(TypeError::InvalidSintax)?;
        let expr = obtener_condicion(cond_not)?;
        return Ok(Expresion::Not(Box::new(expr)));
    }

    let and: Vec<&str> = cond_cls.split(" AND ").collect::<Vec<&str>>();
    if and.len() == 2 {
        let mut exprs = exp(and)?;
        if exprs.len() != 2 {
            return Err(TypeError::InvalidSintax);
        };
        return Ok(Expresion::And(
            Box::new(exprs.remove(0)),
            Box::new(exprs.remove(0)),
        ));
    }

    let or: Vec<&str> = cond_cls.split(" OR ").collect::<Vec<&str>>();
    if or.len() == 2 {
        let mut exprs = exp(or)?;
        if exprs.len() != 2 {
            return Err(TypeError::InvalidSintax);
        };
        return Ok(Expresion::Or(
            Box::new(exprs.remove(0)),
            Box::new(exprs.remove(0)),
        ));
    }

    Ok(Expresion::Condicion(hacer_condition(cond_cls)?))
}

///
/// Evalua la Condicion de busqueda
/// Si no se encuentra la columna a comparar devuelve InvalidColumn
///
fn evaluar(c: &Condicion, index: &str, actual: &str) -> Result<bool, TypeError> {
    let act_vec: Vec<&str> = actual.split(",").collect::<Vec<&str>>();

    let (indice1, indice2) = obtener_indice(index, &c.valores.0,&c.valores.1)?;

    if indice1.is_empty() && indice2.is_empty(){
        return Ok(comparar(&c.valores.0, &c.valores.1, &c.operador))
    }

    let columns: Vec<&str> = index.split(",").collect::<Vec<&str>>();

    if indice1 == c.valores.0 && indice2 != c.valores.1{
        Ok(comparar( &c.valores.1, act_vec[obtener_posicion_indice(&columns, indice1)],&c.operador))
    }else if indice1 != c.valores.0 && indice2 == c.valores.1{
        Ok(comparar(  act_vec[obtener_posicion_indice(&columns, indice1)], &c.valores.0,&c.operador))
    }else{
        Ok(comparar( act_vec[obtener_posicion_indice(&columns, indice2)], act_vec[obtener_posicion_indice(&columns, indice1)],&c.operador))
    }

}

pub fn evaluar_condicion(cond: &Expresion, index: &str, actual: &str) -> Result<bool, TypeError> {
    let actual_cls: String = actual.replace("\n", "");
    let index_cls = index.replace("\n", "");

    let condition: bool = match cond {
        Expresion::Condicion(c) => evaluar(c, &index_cls, &actual_cls)?,
        Expresion::And(c_izq, c_der) => {
            evaluar_condicion(c_izq, &index_cls, &actual_cls)?
                && evaluar_condicion(c_der, &index_cls, &actual_cls)?
        }
        Expresion::Not(c) => !evaluar_condicion(c, &index_cls, &actual_cls)?,
        Expresion::Or(c_izq, c_der) => {
            evaluar_condicion(c_izq, &index_cls, &actual_cls)?
                || evaluar_condicion(c_der, &index_cls, &actual_cls)?
        }
        Expresion::All => true,
    };

    Ok(condition)
}


///
/// Compara los Stings recibidos
///
pub fn cmp_str(compare: &str, actual: &str, operador: &Operador) -> bool {
    match operador {
        Operador::Igual => compare == actual,
        Operador::Mayor => compare < actual,
        Operador::MayorOIgual => compare <= actual,
        Operador::Menor => compare > actual,
        Operador::MenorOIgual => compare >= actual,
        Operador::Error => false,
    }
}

///
/// Compara los enteros recibidos
///
pub fn cmp_int(compare: &isize, actual: &isize, operador: &Operador) -> bool {
    match operador {
        Operador::Igual => compare == actual,
        Operador::Mayor => compare < actual,
        Operador::MayorOIgual => compare <= actual,
        Operador::Menor => compare > actual,
        Operador::MenorOIgual => compare >= actual,
        Operador::Error => false,
    }
}

///
/// Devuelve una tupla con los indices
/// 
fn obtener_indice<'a>(index: &str, valor1: &'a str, valor2: &'a str ) -> Result<(&'a str, &'a str) , TypeError> {

    if index.contains(valor1) && index.contains(valor2){
        Ok((valor1, valor2))
    }else if index.contains(valor1){
        Ok((valor1, valor1))
    }else if index.contains(valor2){
        Ok((valor2, valor2))
    }else if comparar(valor1, valor2, &Operador::Igual){
        println!("a");
        Ok(("", ""))
    }else {
        Err(TypeError::InvalidColumn)
    }
}

///
/// Devuelve la posicion de un elemento en el indice
///
fn obtener_posicion_indice(columns: &Vec<&str>, value: &str) -> usize{
    columns.iter().position(|&x| x == value).unwrap_or(0)
}

///
/// Devuelve true si un &str es numerico
///
fn es_numero(str: &str) -> bool{
    str.chars().all(|ch: char| ch.is_numeric())
}

///
/// Compara 2 &str con el operador pasado por parametro
/// Si los 2 &str se pueden representar como numero los compara numericamente 
///
fn comparar(valor1:&str, valor2:&str, op: &Operador) -> bool{
    if es_numero(valor1) && es_numero(valor2){
        cmp_int(&valor1.parse::<isize>().unwrap_or(0),&valor2.parse::<isize>().unwrap_or(0), op)
    } else {
        cmp_str(valor1, valor2, op)
    }
}

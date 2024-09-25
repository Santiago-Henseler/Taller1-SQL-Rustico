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
    pub column_index: String,
    pub operador: Operador,
    pub value: String,
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
            column_index: strvec[0].to_string(),
            operador: op,
            value: strvec[1].replace("'", "").to_string(),
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
/// Compara los Stings recibidos
///
pub fn cmp_str(compare: &String, actual: &String, operador: &Operador) -> bool {
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
/// Evalua la Condicion de busqueda
/// Si no se encuentra la columna a comparar devuelve InvalidColumn
///
fn evaluar(c: &Condicion, index: &str, actual: &str) -> Result<bool, TypeError> {
    let mut eval = false;
    let act_vec = actual.split(",").collect::<Vec<&str>>();

    let mut column_exist = false;
    for (i, s) in index.replace("\n", "").split(",").enumerate() {
        if *s == c.column_index {
            column_exist = true;
            if c.value.chars().all(|ch: char| ch.is_numeric())
                && act_vec[i].chars().all(|c| c.is_numeric())
            {
                eval = cmp_int(
                    &c.value.parse::<isize>().unwrap_or(0),
                    &act_vec[i].parse::<isize>().unwrap_or(0),
                    &c.operador,
                );
            } else {
                eval = cmp_str(&c.value, &act_vec[i].to_string(), &c.operador);
            }
        }
    }
    if !column_exist {
        return Err(TypeError::InvalidColumn);
    }

    Ok(eval)
}

pub fn evaluar_condicion(cond: &Expresion, index: &str, actual: &str) -> Result<bool, TypeError> {
    let actual_cls = actual.replace("\n", "");

    let condition = match cond {
        Expresion::Condicion(c) => evaluar(c, index, &actual_cls)?,
        Expresion::And(c_izq, c_der) => {
            evaluar_condicion(c_izq, index, &actual_cls)?
                && evaluar_condicion(c_der, index, &actual_cls)?
        }
        Expresion::Not(c) => !evaluar_condicion(c, index, &actual_cls)?,
        Expresion::Or(c_izq, c_der) => {
            evaluar_condicion(c_izq, index, &actual_cls)?
                || evaluar_condicion(c_der, index, &actual_cls)?
        }
        Expresion::All => true,
    };

    Ok(condition)
}

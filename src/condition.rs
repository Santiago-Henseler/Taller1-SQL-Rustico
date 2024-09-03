use crate::query::TypeError;

/// La representación e implementación de las expresiones de busqueda
#[derive(Debug)]
pub enum Operador {
    Mayor,
    MayorOIgual,
    Menor,
    MenorOIgual,
    Igual,
    Error
}

#[derive(Debug)]
pub struct Condicion {
    pub column_index: String,
    pub operador: Operador,
    pub value: String
}

#[derive(Debug)]
pub enum Expresion{
    Condicion(Condicion),
    And((Condicion, Condicion)),
    Or((Condicion, Condicion)),
    Not(Condicion),
    All,
}

/// 
/// Crea la Condicion de busqueda
/// Si no se puede crear devuelve InvalidSintax.
/// 
fn make_condition(str: &String) -> Result<Condicion, TypeError>{

    let strvec: Vec<&str> = str.trim().split_whitespace().collect::<Vec<&str>>();

    if strvec.len() != 3{
        return Err(TypeError::InvalidSintax)
    }

    let op: Operador = match strvec[1] {
        "="  => Operador::Igual,
        ">=" => Operador::MayorOIgual,
        "<=" => Operador::MenorOIgual,
        "<"  => Operador::Menor,
        ">"  => Operador::Mayor,
        _    => Operador::Error 
    };

    match op{
        Operador::Error => Err(TypeError::InvalidSintax),
        _ => Ok(Condicion{
            column_index: strvec[0].to_string(),
            operador: op,
            value: strvec[2].to_string()
        }),
    }
}

/// 
/// Crea la Expresion de busqueda
/// Si no se puede crear devuelve InvalidSintax.
/// 
pub fn get_conditions(condition: &str) -> Result<Expresion, TypeError>{

    if condition.contains("AND"){
        let str: Vec<&str> = condition.split("AND").collect::<Vec<&str>>();
        Ok(Expresion::And((make_condition(&str[0].to_string())?, make_condition(&str[1].to_string())?)))
    }else if condition.contains("NOT"){
        let str: Vec<&str> = condition.split("NOT").collect::<Vec<&str>>();
        Ok(Expresion::Not(make_condition(&str[1].to_string())?))
    }else if condition.contains("OR"){
        let str: Vec<&str> = condition.split("OR").collect::<Vec<&str>>();
        Ok(Expresion::Or((make_condition(&str[0].to_string())?, make_condition(&str[1].to_string())?)))
    }else{
       Ok(Expresion::Condicion(make_condition(&condition.to_string())?))
    }
}

/// 
/// Compara los Stings recibidos
/// 
fn cmp_str(compare:&String, actual: &String, operador:&Operador) -> bool{
    let eval = match operador {
        Operador::Igual => compare == actual,
        Operador::Mayor => compare < actual,
        Operador::MayorOIgual => compare <= actual,
        Operador::Menor => compare > actual,
        Operador::MenorOIgual => compare >= actual,
        Operador::Error => false,
    };
    eval
}

/// 
/// Compara los enteros recibidos
/// 
fn cmp_int(compare:&isize, actual: &isize, operador:&Operador) -> bool{
    let eval = match operador {
        Operador::Igual => compare == actual,
        Operador::Mayor => compare < actual,
        Operador::MayorOIgual => compare <= actual,
        Operador::Menor => compare > actual,
        Operador::MenorOIgual => compare >= actual,
        Operador::Error => false,
    };
    eval
}

/// 
/// Evalua la Condicion de busqueda
/// Si no se encuentra la columna a comparar devuelve InvalidColumn
/// 
pub fn evaluar(c: &Condicion, index:&String, actual: &String) -> Result<bool, TypeError>{

    let mut eval = false;
    let act_vec = actual.split(",").collect::<Vec<&str>>();

    let mut column_exist = false;
    for (i, s) in index.replace("\n", "").split(",").enumerate(){
        if s.to_string() == c.column_index{
            column_exist = true;
            if c.value.chars().all(|ch: char| ch.is_numeric()) && act_vec[i].chars().all(|c| c.is_numeric()){
                eval = cmp_int(&c.value.parse::<isize>().unwrap_or(0), &act_vec[i].parse::<isize>().unwrap_or(0), &c.operador);
            }else{
                eval = cmp_str(&c.value, &act_vec[i].to_string(), &c.operador);
            }
        }
    }
    if !column_exist{
        return  Err(TypeError::InvalidColumn)
    }

    Ok(eval)
}

#[derive(Debug)]
pub enum Operador {
    Mayor,
    MayorOIgual,
    Menor,
    MenorOIgual,
    Igual
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
    Not(Condicion)
}

fn make_condition(str: &String) -> Condicion{

    let strvec: Vec<&str> = str.trim().split_whitespace().collect::<Vec<&str>>();

    let op = match strvec[1] {
        "="  => Operador::Igual,
        ">=" => Operador::MayorOIgual,
        "<=" => Operador::MenorOIgual,
        "<"  => Operador::Menor,
        ">"  => Operador::Mayor,
        _    => Operador::Igual 
    };

    Condicion{
        column_index: strvec[0].to_string(),
        operador: op,
        value: strvec[2].to_string()
    }
}

pub fn get_conditions(condition: &str) -> Expresion {

    if condition.contains("AND"){
        let str: Vec<&str> = condition.split("AND").collect::<Vec<&str>>();
        Expresion::And((make_condition(&str[0].to_string()), make_condition(&str[1].to_string())))
    }else if condition.contains("NOT"){
        let str: Vec<&str> = condition.split("NOT").collect::<Vec<&str>>();
        Expresion::Not(make_condition(&str[1].to_string()))
    }else if condition.contains("OR"){
        let str: Vec<&str> = condition.split("OR").collect::<Vec<&str>>();
        Expresion::Or((make_condition(&str[0].to_string()), make_condition(&str[1].to_string())))
    }else{
        Expresion::Condicion(make_condition(&condition.to_string()))
    }
}

pub fn evaluar(c: &Condicion, index:&String, actual: &String) -> bool{

    let mut eval = false;
    let act_vec = actual.split(",").collect::<Vec<&str>>();

    for (i, s) in index.replace("\n", "").split(",").enumerate(){
        if s.to_string() == c.column_index{
            eval = match c.operador {
                Operador::Igual => c.value == act_vec[i].to_string(),
                Operador::Mayor => c.value > act_vec[i].to_string(),
                Operador::MayorOIgual => c.value >= act_vec[i].to_string(),
                Operador::Menor => c.value < act_vec[i].to_string(),
                Operador::MenorOIgual => c.value <= act_vec[i].to_string(),
            }
        }
    }
    eval
}
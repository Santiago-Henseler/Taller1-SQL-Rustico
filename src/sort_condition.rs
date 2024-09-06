use crate::query;
use query::TypeError;

/// La representación e implementación de las condiciones de ordenamiento
#[derive(Debug)]
pub struct SortCondition {
    pub column_index: String,
    pub ascendiente: bool,
    pub second_condition: String,
}

#[derive(Debug)]
pub enum SortExpresion {
    SortCondition(SortCondition),
    None,
}

/// 
/// Crea la SortCondition
/// Si no se puede crear devuelve InvalidSintax.
/// 
fn make_sort_condition(first_cond:&str, second_cond: String) -> Result<SortCondition, TypeError>{

    let first_cond_vec = first_cond.split_whitespace().collect::<Vec<&str>>();
    let mut ascendente = true;

    if first_cond_vec.len() == 2 {
        ascendente = match first_cond_vec[1] {
                "ASC" => true,
                "DESC"=> false,
                _ => return Err(TypeError::InvalidSintax),
            };
    }else if first_cond_vec.len() != 1 {
        return Err(TypeError::InvalidSintax)
    }

    Ok(SortCondition {
        column_index: first_cond_vec[0].to_string(),
        ascendiente: ascendente,
        second_condition: second_cond,
    })
}

/// 
/// Crea la SortExpresion
/// Si no se puede crear devuelve InvalidSintax.
/// 
pub fn make_sort_expresion(str:&str)-> Result<SortExpresion, TypeError>{

    let cond_vec: Vec<&str> = str.split(',').collect::<Vec<&str>>();
    let mut second_cond = String::from("");

    if cond_vec.len() == 2{
        second_cond = cond_vec[1].trim().to_string();
    }

    Ok(SortExpresion::SortCondition(make_sort_condition(cond_vec[0], second_cond)?))
}

/// 
/// Determina si es mayor o menor
/// 
fn es_mayor(ascendete:bool, mayor: &str, actual: &str) -> bool{
    if ascendete{
        if mayor > actual{
            return true
        }
    }else if mayor < actual{
            return true
        }
    false
}

///
/// Determina si la segunda condicion es mayor
/// 
fn es_mayor_segcond(second_cond: &String, lines: &[String], max: usize, j:usize) -> Result<bool, TypeError>{

    let column :isize = match lines[0].split(',').collect::<Vec<&str>>().iter().position(|x| x == second_cond){
        Some(n) => n as isize,
        None => return Err(TypeError::InvalidSintax),
    };

    let mayor: &str = lines[max].split(',').collect::<Vec<&str>>()[column as usize];
    let actual = lines[j].split(',').collect::<Vec<&str>>()[column as usize];

    Ok(es_mayor(false, mayor, actual))
}

/// 
/// Apartir de la SortCondition ordena las filas
/// Si no se puede ordenar devuelve InvalidSintax.
/// 
pub fn sort(lines: &mut [String], conds_exp: &SortExpresion) -> Result<(), TypeError>{
    let conds: &SortCondition = match conds_exp{
        SortExpresion::SortCondition(c) => c,
        SortExpresion::None => return Err(TypeError::InvalidSintax),
    };
    let column :isize = match lines[0].split(',').collect::<Vec<&str>>().iter().position(|&x| x == conds.column_index){
        Some(n) => n as isize,
        None => return Err(TypeError::InvalidSintax),
    };
    for i in 1..lines.len(){
        let mut max = i;
        for j in i+1..lines.len(){
            let mayor: &str = lines[max].split(',').collect::<Vec<&str>>()[column as usize];
            let actual = lines[j].split(',').collect::<Vec<&str>>()[column as usize];
            if mayor == actual{
                if es_mayor_segcond(&conds.second_condition, lines, max, j)?{max = j;}
            }
            else if es_mayor(conds.ascendiente, mayor, actual){max = j;}
        }
        let aux = lines[i].clone();
        lines[i] = lines[max].clone();
        lines[max] = aux;
    }
    Ok(())
}
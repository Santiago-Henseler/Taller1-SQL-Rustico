use crate::query;
use query::TypeError;

/// La representación e implementación de las condiciones de ordenamiento
#[derive(Debug)]
pub struct SortCondition {
    pub column_index: String,
    pub ascendiente: bool,
    pub seg_condition: String,
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
pub fn make_sort_condition(str:&str) -> Result<SortExpresion, TypeError>{

    let vstr = str.split_whitespace().collect::<Vec<&str>>();
    let mut ascendente = true;

    if vstr.len() == 2 {
        ascendente = match vstr[1] {
                "ASC" => true,
                "DESC"=> false,
                _ => return Err(TypeError::InvalidSintax),
            };
    }else if vstr.len() != 1 {
        return Err(TypeError::InvalidSintax)
    }

    Ok(SortExpresion::SortCondition(SortCondition {
        column_index: vstr[0].to_string(),
        ascendiente: ascendente,
        seg_condition: "".to_string(),
    }))
}

/// 
/// Determina si es mayor o menor
/// 
fn es_mayor(ascendete:bool, mayor: &str, actual: &str) -> bool{
    if ascendete{
        if mayor > actual{
            return true
        }
    }else{
        if mayor < actual{
            return true
        }
    }
    false
}

/// 
/// Apartir de la SortCondition ordena las filas
/// Si no se puede ordenar devuelve InvalidSintax.
/// 
pub fn sort(lines: &mut Vec<String>, conds_exp: &SortExpresion) -> Result<(), TypeError>{
    let conds: &SortCondition = match conds_exp{
        SortExpresion::SortCondition(c) => c,
        SortExpresion::None => return Err(TypeError::InvalidSintax),
    };
    let column :isize = match lines[0].as_str().split(',').collect::<Vec<&str>>().iter().position(|&x| x == conds.column_index){
        Some(n) => n as isize,
        None => return Err(TypeError::InvalidSintax),
    };
    for i in 1..lines.len(){
        let mut max = i;
        for j in i+1..lines.len(){
            let mayor: &str = lines[max].split(',').collect::<Vec<&str>>()[column as usize];
            let actual = lines[j].split(',').collect::<Vec<&str>>()[column as usize];
            if es_mayor(conds.ascendiente, mayor, actual){
                max = j;
            }
        }
        let aux = lines[i].clone();
        lines[i] = lines[max].clone();
        lines[max] = aux;
    }
    Ok(())
}
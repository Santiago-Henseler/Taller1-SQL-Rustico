use crate::query;
use query::TypeError;

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

// Agregar segunda condicion 
pub fn make_sort_condition(str:&str) -> Result<SortExpresion, TypeError>{

    let vstr = str.split_whitespace().collect::<Vec<&str>>();

    if vstr.len() != 2{
        return Err(TypeError::InvalidSintax)
    }

    Ok(SortExpresion::SortCondition(SortCondition {
        column_index: vstr[0].to_string(),
        ascendiente: match vstr[1] {
            "ASC" => true,
            "DESC"=> false,
            _ => true,
         },
        seg_condition: "".to_string(),
    }))
}

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

pub fn sort(lines: &mut Vec<String>, conds_exp: &SortExpresion) -> Result<(), TypeError>{
    let conds = match conds_exp{
        SortExpresion::SortCondition(c) => c,
        SortExpresion::None => &SortCondition { column_index: "".to_string(), ascendiente: true, seg_condition: "".to_string()},
    };
    let column :isize = match lines[0].as_str().split(',').collect::<Vec<&str>>().iter().position(|&x| x == conds.column_index){
        Some(n) => n as isize,
        None => -1,
    };
    if column == -1 || conds.column_index == ""{
        return Err(TypeError::InvalidSintax)
    }
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
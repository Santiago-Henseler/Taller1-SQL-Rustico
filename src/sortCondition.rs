
#[derive(Debug)]
pub struct SortCondition {
    pub column_index: String,
    pub ascendiente: bool,
}

#[derive(Debug)]
pub enum SortExpresion {
    SortCondition(SortCondition),
    None,
}

pub fn  make_sort_condition(str:&str) -> SortExpresion{

    let vstr = str.split_whitespace().collect::<Vec<&str>>();

    SortExpresion::SortCondition(SortCondition {
         column_index: vstr[0].to_string(), ascendiente: match vstr[1] {
            "ASC" => true,
             _=> false,
         } 
    })
}
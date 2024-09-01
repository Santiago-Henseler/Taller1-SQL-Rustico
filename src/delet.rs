use crate::query;
use query::Query;
use query::TypeError;
use crate::condition;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;

#[derive(Debug)]
pub struct Delet{
    conditions: Expresion,
}

impl Delet{
    pub fn new(table:String, query: &String) -> Result<Self, TypeError>{
        
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>();

        if !query.contains("WHERE"){
            return Err(TypeError::InvalidSintax)
        }
        let conditions =  get_conditions(str[1].split("WHERE").collect::<Vec<&str>>()[1].replace(',', " AND ").as_str())?;

        Ok(Self {
            conditions: conditions,
        })
    }
}

impl Query for Delet{
    fn operate(&mut self, index:&String, actual:String) -> String{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", "")),
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", "")) && evaluar(c_der, index, &actual.replace("\n", "")),
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", "")),
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", "")) || evaluar(c_der, index, &actual.replace("\n", "")),
        };

        match condition{
            true => "".to_string(),
            false => actual,
        }
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("DELET FROM tabla1 id_cliente = 1 ");
    let try1: Result<Delet, TypeError>  = Delet::new("tabla1".to_string(), &str1);
    
    match try1{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test2(){
    let str2 = String::from("DELET FROM tabla1 WHERE  ");
    let try2: Result<Delet, TypeError>  = Delet::new("tabla1".to_string(), &str2);
    
    match try2{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test3(){
    let str3 = String::from("DELET FROM tabla1 WHERE = 1 ");
    let try3: Result<Delet, TypeError>  = Delet::new("tabla1".to_string(), &str3);
    
    match try3{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn operate_test1(){
    let str = String::from("DELET FROM tabla1 WHERE id_cliente = 1");
    let mut instance:Delet  = Delet::new("tabla1".to_string(), &str).unwrap();

    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1".to_string());

    assert_eq!(word, "".to_string());
}

#[test]
fn operate_test2(){
    let str = String::from("Delet tabla1 SET id = 99 WHERE id_cliente = 3");
    let mut instance:Delet  = Delet::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id, id_cliente, producto, cantidad".to_string(), "101,1,Laptop,1".to_string());

    assert_eq!(word, "101,1,Laptop,1".to_string());
}
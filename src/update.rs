use crate::query;
use crate::condition;
use query::Query;
use query::TypeError;
use condition::get_conditions;
use condition::Expresion;
use condition::evaluar;

#[derive(Debug)]
pub struct Update{
    conditions: Expresion,
    set: Expresion,
}

impl Update{
    pub fn new(table:String, query: &String) -> Result<Self, TypeError>{

        if !query.contains("WHERE") || !query.contains("SET"){
            return Err(TypeError::InvalidSintax)
        }
        let str: Vec<&str> = query.split(&table).collect::<Vec<&str>>()[1].split("WHERE").collect::<Vec<&str>>();

        let conditions = get_conditions(str[1].replace(',', " AND ").as_str())?;
        let set = get_conditions(str[0].replace(',', " AND ").split("SET").collect::<Vec<&str>>()[1])?;

        Ok(Self {
            conditions: conditions,
            set: set
        })
    }
}

fn update_str(expresion: &Expresion, index: &String, actual: String) -> String{

    let mut new_string: String = String::from("");

    match expresion{
        Expresion::Condicion(c) => {
            let act_vec = actual.split(",").collect::<Vec<&str>>();
            for (i, s) in index.replace("\n", "").split(",").enumerate(){
                if s.to_string() == c.column_index{
                    new_string.push_str(&c.value.replace("'", ""));
                }else{
                    new_string.push_str(act_vec[i]);
                }
                new_string.push(',');
            }
            new_string.pop();
            },
        _ =>(),
    }


    println!("a");
    new_string
}

impl Query for Update{
    fn operate(&mut self, index:&String, actual:String) -> String{

        let condition: bool = match &self.conditions{
            Expresion::Condicion(c) => evaluar(c, index, &actual.replace("\n", "")),
            Expresion::And((c_izq, c_der)) => evaluar(c_izq, index, &actual.replace("\n", "")) && evaluar(c_der, index, &actual.replace("\n", "")),
            Expresion::Not(c) => !evaluar(c, index, &actual.replace("\n", "")),
            Expresion::Or((c_izq, c_der))=> evaluar(c_izq, index, &actual.replace("\n", "")) || evaluar(c_der, index, &actual.replace("\n", "")),
        };

        match condition{
            true => update_str(&self.set, index, actual),
            false => actual,
        }
    }
}

#[test]
fn sintax_error_test1() {
    let str1 = String::from("UPDATE tabla1 id = 99 WHERE id_cliente = 1 ");
    let try1: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str1);
    
    match try1{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test2(){
    let str2 = String::from("UPDATE tabla1 SET WHERE id_cliente = 1");
    let try2: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str2);
    
    match try2{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test3(){
    let str3 = String::from("UPDATE tabla1 SET id = 99  id_cliente = 1");
    let try3: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str3);
    
    match try3{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn sintax_error_test4(){
    let str4 = String::from("UPDATE tabla1 SET id = 99 WHERE dadadad");
    let try4: Result<Update, TypeError>  = Update::new("tabla1".to_string(), &str4);
    
    match try4{ 
        Err(TypeError::InvalidSintax) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn operate_test1(){
    let str = String::from("UPDATE tabla1 SET id = 99 WHERE id_cliente = 1");
    let mut instance:Update  = Update::new("tabla1".to_string(), &str).unwrap();

    let word = instance.operate(&"id,id_cliente,producto,cantidad".to_string(), "101,1,Laptop,1".to_string());

    assert_eq!(word, "99,1,Laptop,1".to_string());
}

#[test]
fn operate_test2(){
    let str = String::from("UPDATE tabla1 SET id = 99 WHERE id_cliente = 3");
    let mut instance:Update  = Update::new("tabla1".to_string(), &str).unwrap();
    
    let word = instance.operate(&"id, id_cliente, producto, cantidad".to_string(), "101,1,Laptop,1".to_string());

    assert_eq!(word, "101,1,Laptop,1".to_string());
}
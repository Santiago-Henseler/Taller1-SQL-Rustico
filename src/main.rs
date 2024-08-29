use std::env;

mod insert;
mod delet;
mod update;
mod query;

use delet::Delet;
use update::Update;
use insert::Insert;
use query::Query;
use query::TypeError;

// TODO:
// Agregar casos de error en los distintas cosas :)
// Crear el SELECT
// agregar utilidades a la clase query
fn parsear(query: &String) -> Result<Box<dyn Query>, TypeError>{

    let vec: Vec<&str> =  query.split(' ').collect::<Vec<&str>>();

    match vec[0] {
        "INSERT" => {
            let instance = Insert::new(vec[2].to_string(), query);
            Ok(Box::new(instance))
        },
        "UPDATE" => {
            let instance = Update::new(vec[2].to_string(), query);
            Ok(Box::new(instance))
        },
        "DELET"  => {
            let instance = Delet::new(vec[2].to_string(), query);
            Ok(Box::new(instance))
        },
        //"SELECT" => Select::new(vec[2].to_string(), query)?,
        _ => Err(TypeError::InvalidSintax),
    }

}

fn main()-> Result<(), TypeError>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("INVALID_SYNTAX: El comando debe correrse como 'cargo run -- ruta/a/tablas <consulta>'" );
    }

    let a:Box<dyn Query>  = parsear(&args[2])?;

    a.operate("id,id_cliente,producto,cantidad".to_string(), "5".to_string());

    Ok(())
}

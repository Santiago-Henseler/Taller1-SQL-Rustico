use std::env;

mod insert;
mod delet;
mod update;
mod query;
mod condition;

use delet::Delet;
use update::Update;
use insert::Insert;
use insert::insert_reg;
use query::mod_file;
use query::TypeError;

// TODO:
// Agregar casos de error en los distintas cosas :)
// Crear el SELECT
// Terminar el update
// modificar utilidades a la clase query

fn get_path(archivo: &str, dir:&String) -> String{

    let mut path: String = String::from("");
    path.push_str(dir);
    path.push('/');
    path.push_str(archivo);
    path.push_str(".csv");
    path
}

fn run(query: &String, dir:String) -> Result<(), TypeError>{

    let vec: Vec<&str> = query.trim().split_whitespace().collect::<Vec<&str>>();

    match vec[0] {
        "INSERT" => {
            let path = get_path(vec[2], &dir);
            let instance = Insert::new(vec[2].to_string(), query);
            insert_reg(path, instance)?;
            Ok(())
        },
        "UPDATE" => {
            let path = get_path(vec[1], &dir);
            let instance = Update::new(vec[1].to_string(), query);
            mod_file(path, Box::new(instance))?;
            Ok(())
        },
        "DELETE" => {
            let path = get_path(vec[2], &dir);
            let instance = Delet::new(vec[2].to_string(), query);
            mod_file(path, Box::new(instance))?;
            Ok(())
        },
        "SELECT" => {
           // let path = get_path(vec[2], &dir);
           //let instance = Delet::new(vec[2].to_string(), query);
           // mod_file(path, Box::new(instance))?;
            Ok(())
        },
        _ => Err(TypeError::InvalidSintax),
    }

}

fn main()-> Result<(), TypeError>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("INVALID_SYNTAX: El comando debe correrse como 'cargo run -- ruta/a/tablas <consulta>'" );
    }
    
    run(&args[2], args[1].to_owned())
}

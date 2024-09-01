use std::env;

mod insert;
mod delet;
mod update;
mod select;
mod query;
mod condition;
mod sort_condition;

use delet::Delet;
use update::Update;
use insert::Insert;
use insert::insert_reg;
use select::Select;
use query::mod_file;
use query::TypeError;

// TODO:
// Agregar casos de error en los distintas cosas :)
// SELECT algunas columnas
// Agregar segunda condicion de sort 
// modificar utilidades a la clase query
// mejorar el get path
// hacer test y documentar codigo

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
            let mut instance: Insert = Insert::new(vec[2].to_string(), query)?;
            insert_reg(path, &mut instance)?;
            Ok(())
        },
        "UPDATE" => {
            let path = get_path(vec[1], &dir);
            let mut instance: Update = Update::new(vec[1].to_string(), query)?;
            mod_file(path, &mut instance )?;
            Ok(())
        },
        "DELETE" => {
            let path = get_path(vec[2], &dir);
            let mut instance: Delet = Delet::new(vec[2].to_string(), query)?;
            mod_file(path,&mut instance )?;
            Ok(())
        },
        "SELECT" => {
            let path = get_path(vec[3], &dir);
            let mut instance: Select = Select::new(vec[3].to_string(), query);
            mod_file(path, &mut instance)?;
            instance.print()?;
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

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



fn get_path(archivo: &str, dir:&str) -> String{

    let mut path: String = String::from("");
    path.push_str(dir);
    path.push('/');
    path.push_str(archivo);
    path.push_str(".csv");

    path
}

fn run(query: &str, dir:String) -> Result<(), TypeError>{

    let vec: Vec<&str> = query.split_whitespace().collect::<Vec<&str>>();

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
            let tabla = query.split("FROM").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>()[0];
            let path = get_path(tabla, &dir);
            let mut instance: Select = Select::new(tabla.to_string(), query)?;
            mod_file(path, &mut instance)?;
            instance.print()?;
            Ok(())
        },
        _ => Err(TypeError::InvalidSintax),
    }
}

fn show_error(query: &str){
    let vec: Vec<&str> = query.split_whitespace().collect::<Vec<&str>>();

    match vec[0] {
        "INSERT" => {
            println!("El comando se debe ejecutar como 'INSERT INTO tabla (col_0, col_1, col_2, col3) VALUES (new_0, new_1, new_2, new_3)'")
        },
        "UPDATE" => {
            println!("El comando se debe ejecutar como 'UPDATE tabla SET col_0 = new_value WHERE col_1 = value'")
        },
        "DELETE" => {   
            println!("El comando se debe ejecutar como 'DELETE FROM tabla WHERE col_1 = value'")
        },
        "SELECT" => {
            println!("El comando se debe ejecutar como 'SELECT col_0, col_1 FROM tabla WHERE col_1 = value ORDER BY col_1 ASC'")
            
        },
        _ => println!("Comando inexistente los comandos son SELECT UPDATE DELETE INSERT"),
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("InvalidSintax: El comando debe correrse como 'cargo run -- ruta/a/tablas <consulta>'" );
        return;
    }

    match run(&args[2].replace("\n", ""), args[1].to_owned()){
        Err(TypeError::InvalidSintax) => {print!("InvalidSintax: "); show_error(&args[2].replace("\n", ""))},
        Err(TypeError::Error) => println!("Error: ocurrio un problema al abri/leer/escribir un archivo intentelo nuevamente"),
        Err(TypeError::InvalidColumn) => println!("InvalidColumn: la columna no existe en la tabla"),
        Err(TypeError::InvalidaTable) => println!("InvalidaTable: la tabla no fue encontrada"),
        _ => (),
    }
}

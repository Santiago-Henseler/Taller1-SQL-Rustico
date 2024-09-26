use std::env;

mod condition;
mod delet;
mod insert;
mod query;
mod select;
mod sort_condition;
mod update;

use delet::Delete;
use insert::agregar_reg;
use insert::Insert;
use query::mod_file;
use query::TypeError;
use select::Select;
use update::Update;

fn get_path(archivo: &str, dir: &str) -> String {
    let mut path: String = String::from("");
    path.push_str(dir);
    path.push('/');
    path.push_str(archivo);
    path.push_str(".csv");

    path
}

fn insert(tabla: &str, dir: &str, query: &str) -> Result<(), TypeError> {
    let path = get_path(tabla, dir);
    let mut instance: Insert = Insert::new(tabla.to_string(), query)?;
    agregar_reg(path, &mut instance)?;
    Ok(())
}

fn update(tabla: &str, dir: &str, query: &str) -> Result<(), TypeError> {
    let path = get_path(tabla, dir);
    let mut instance: Update = Update::new(tabla.to_string(), query)?;
    mod_file(path, &mut instance)?;
    Ok(())
}

fn delete(tabla: &str, dir: &str, query: &str) -> Result<(), TypeError> {
    let path = get_path(tabla, dir);
    let mut instance: Delete = Delete::new(tabla.to_string(), query)?;
    mod_file(path, &mut instance)?;
    Ok(())
}

fn select(dir: &str, query: &str) -> Result<(), TypeError> {
    if let Some(tb) = query.split(" FROM ").collect::<Vec<&str>>().get(1) {
        if let Some(tabla) = tb.split_whitespace().collect::<Vec<&str>>().first() {
            let path = get_path(tabla, dir);
            let mut instance: Select = Select::new(tabla.to_string(), query)?;
            mod_file(path, &mut instance)?;
            instance.print()?;
            Ok(())
        } else {
            Err(TypeError::InvalidSintax)
        }
    } else {
        Err(TypeError::InvalidSintax)
    }
}

fn ejecutar(query: &str, dir: &str) -> Result<(), TypeError> {
    let vec: Vec<&str> = query.split_whitespace().collect::<Vec<&str>>();

    if let Some(comando) = vec.first() {
        match comando.to_uppercase().as_str() {
            "INSERT" => {
                if let Some(tabla) = vec.get(2) {
                    insert(tabla, dir, query)
                } else {
                    Err(TypeError::InvalidSintax)
                }
            }
            "UPDATE" => {
                if let Some(tabla) = vec.get(1) {
                    update(tabla, dir, query)
                } else {
                    Err(TypeError::InvalidSintax)
                }
            }
            "DELETE" => {
                if let Some(tabla) = vec.get(2) {
                    delete(tabla, dir, query)
                } else {
                    Err(TypeError::InvalidSintax)
                }
            }
            "SELECT" => select(dir, query),
            _ => Err(TypeError::InvalidSintax),
        }
    } else {
        Err(TypeError::InvalidSintax)
    }
}

fn show_error(query: &str) {
    let vec: Vec<&str> = query.split_whitespace().collect::<Vec<&str>>();

    if let Some(comando) = vec.first() {
        match *comando {
            "INSERT" => {
                println!("El comando se debe ejecutar como 'INSERT INTO tabla (col_0, col_1, col_2, col3) VALUES (new_0, new_1, new_2, new_3)'")
            }
            "UPDATE" => {
                println!("El comando se debe ejecutar como 'UPDATE tabla SET col_0 = new_value WHERE col_1 = value'")
            }
            "DELETE" => {
                println!("El comando se debe ejecutar como 'DELETE FROM tabla WHERE col_1 = value'")
            }
            "SELECT" => {
                println!("El comando se debe ejecutar como 'SELECT col_0, col_1 FROM tabla WHERE col_1 = value ORDER BY col_1 ASC'")
            }
            _ => println!("Comando inexistente los comandos son SELECT UPDATE DELETE INSERT"),
        }
    } else {
        println!("Comando inexistente los comandos son SELECT UPDATE DELETE INSERT")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "InvalidSintax: El comando debe correrse como 'cargo run -- ruta/a/tablas <consulta>'"
        );
        return;
    }

    match ejecutar(&args[2].replace("\n", " "), &args[1]) {
        Err(TypeError::InvalidSintax) => {
            print!("InvalidSintax: ");
            show_error(&args[2].replace("\n", ""))
        }
        Err(TypeError::Error) => println!(
            "Error: ocurrio un problema al abri/leer/escribir un archivo intentelo nuevamente"
        ),
        Err(TypeError::InvalidColumn) => {
            println!("InvalidColumn: la columna no existe en la tabla")
        }
        Err(TypeError::InvalidaTable) => println!("InvalidaTable: la tabla no fue encontrada"),
        _ => (),
    }
}

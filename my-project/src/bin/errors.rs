use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    println!("hello world!");
    _do_result();
    //_do_panic();
}

fn _do_panic() {
    panic!("Manolo Escobar agarra la guitarra!");
}

fn _do_result() {
    let greeting_file_result = File::open("hello.text");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("fichero no encontrado"),
            _ => panic!("que has tocado?"),
        }
    };

    let _greeting_file = File::open("equvalente.txt").unwrap(); // equivalente a lo anterior, da el file o panica
    let _greeting_file = File::open("con mensaje.txt").expect("fichero no encontrado"); //equivalente con mensaje para el panic
}

fn _escalo_error() -> Result<String, io::Error> {
    let username_file_result = File::open("hellow.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return  Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _escalo_error_sugar() -> Result<String, io::Error> {
    let mut username_file = File::open("fail.txt")?; //if err return err, pasa por el trait from, para cambiar el tipo
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _escalo_error_diabetes() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("fail_again.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _escalo_error_god() -> Result<String, io::Error> {
    std::fs::read_to_string("faillikagod.txt")
}


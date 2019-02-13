use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

fn setup() {
    File::create("a")
        .and_then(|mut file| file.write_all(b"grape"))
        .unwrap();
    File::create("b")
        .and_then(|mut file| file.write_all(b"fruit"))
        .unwrap();
}

fn get_data(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
}

fn concat(filename_a: &str, filename_b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(filename_a), get_data(filename_b));
    data_a.and_then(
        |a| {
            data_b.and_then(
                |b| {
                    Ok(a + &b)
                })
        }
    )
}

fn main() {
    setup();
    match concat("a", "b") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("Error: {}", e)
    }
}
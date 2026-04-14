use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn leggi_file(path: &str) -> Result<Vec<String>, &'static str>{

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err("File non trovato")
    };
    let reader = BufReader::new(file);

    let mut rows: Vec<String> = Vec::new();

    for row in reader.lines(){
        let r = match row {
            Ok(row) => row,
            Err(_) => return Err("Errore in lettura")
        };
        rows.push(r);
    }
    Ok(rows)
}

pub fn conta_righe(righe: &[String]) -> usize{
    righe.len()
}
pub fn stampa_head(righe: &[String], num: usize) {
    for riga in righe.iter().take(num) {
        println!("{}", riga);
    } 
    
}

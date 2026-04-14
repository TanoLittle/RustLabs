use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileData{
    pub testo: Vec<String>,
    pub total_lines: i32,
}

pub fn leggi_file(percorso: &String, lines: usize) -> Result<FileData, &'static str>{

    let file = match File::open(percorso){
        Ok(f) => f,
        Err(_) => return Err("non si può"),
    };

    let reader = BufReader::new(file);

    let mut total_lines = 0;
    let mut testo: Vec<String> = Vec::new();

    for riga_result in reader.lines(){
        total_lines += 1;

        match riga_result{

            Ok(testo_riga) => {
                if testo.len() < lines {
                    testo.push(testo_riga);
                }
            }

            Err(_) => return Err("non si può"),

        }
    }

    Ok(FileData { testo, total_lines })

}
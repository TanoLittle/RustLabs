mod io;
mod cli;

use std::env::args;
use std::process;

fn main() {

    let args: Vec<String> = args().collect();


    let specs = match cli::parse_args(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Errore negli argomenti: {}", e);
            process::exit(1);
        }
    }; 

    match io::leggi_file(&specs.nome_file) {
        Ok(righe) => {

            let totale = io::conta_righe(&righe);
            println!("rows: {}", totale);
            io::stampa_head(&righe, specs.num_lines);
        }
        Err(e) => {
            eprintln!("Errore durante l'accesso al file '{}': {}", specs.nome_file, e);
            process::exit(1);
        }
    }


}

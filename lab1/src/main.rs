mod cli;
mod io;

fn main() {
    let args = cli::parse_argomenti();
    
    println!("{:?}", args);

    // Primo bivio: controlliamo gli argomenti
    let parametri = match args {
        Ok(p) => p,
        Err(_) => {
            println!("Errore: parametri mancanti o errati.");
            return; // Termina il main e chiude il programma
        }
    };

    let risultato_file = io::leggi_file(&parametri.file_name, parametri.num_lines);

    // Secondo bivio: controlliamo la lettura del file
    let file_data = match risultato_file {
        Ok(dati) => dati,
        Err(_) => {
            println!("Errore: impossibile leggere il file.");
            return; // Termina il main e chiude il programma
        }
    };

    // Se arriviamo qui, è andato tutto bene
    println!("rows: {}", file_data.total_lines);
    println!("head ({}):", parametri.num_lines);
    for riga in file_data.testo {
        println!("{}", riga);
    }
}

#[derive(Debug)]
pub struct Arguments{
    pub nome_file: String,
    pub num_lines: usize
}


pub fn parse_args(args: Vec<String>) -> Result<Arguments, &'static str>{
   
    
    if args.len() < 2 {
        return Err("Missing parameters");
    }
    
    let nome_file = args[1].clone();
    let mut num_lines: usize = 10;

    if args.len() > 2 {
        if args[2] == "--head"{
            if args.len() > 3 {
                match args[3].parse::<usize>() {
                    Ok(n) => num_lines = n,
                    Err(_) => return Err("Il parametro per --head non è un numero"),
                }
            } else {
                return Err("Valore mancante per l'argomento --head");
            }
        } else {
            return Err("Argomento non riconosciuto");   
        }
    }

    Ok(Arguments { nome_file, num_lines })
    
}
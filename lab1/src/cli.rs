use std::env::args;

#[derive (Debug)]
pub struct Argument{
    pub file_name: String,
    pub num_lines: usize,
}

pub fn parse_argomenti() -> Result<Argument, &'static str>{

    let args: Vec<String> = args().collect();

    println!("{:?}", args);

    if args.len() < 2 {
        return Err("missing parameters");
    }

    let file_name = args[1].clone();
    let mut num_lines: usize = 10;
   
    if args.len() > 3{
        if args[2] == "--head"{
        match args[3].parse::<usize>(){
            Ok(n) => num_lines = n,
            Err(_) => return Err("Not a number"),
        }
    }
    else {
        return Err("mising --head parameter");
    }
        
    }
    
    Ok(Argument{file_name, num_lines})

}
    
    
    
    



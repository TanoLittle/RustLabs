mod table;
use std::env;
use table::{SUBS_I, SUBS_O};




fn slugify(s: &str) -> String {
    let mut result = String::new();
    
    for c in s.chars() {
        for ch in c.to_lowercase() {
            let converted = conv(ch);
            
            // Skip duplicate consecutive dashes
            if converted == '-' && result.chars().last() == Some('-') {
                continue;
            }
            
            result.push(converted);
        }
    }
    
    // Remove trailing dash
    if result.ends_with('-') && result.len() > 1 {
        result.pop();
    }
    
    result
}

fn conv ( c : char ) -> char {
    if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
        return c;
    }

    else {
        if let Some(index) = SUBS_I.find(c){
            return SUBS_O.chars().nth(index).unwrap();
        }
    }

    '-'

}

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() == 0 {
        println!("No arguments provided.");
        return;
    }
    else if args.len() > 1 {
        println!("Too many arguments provided.");
        return;  
    }

    let input = args[0].clone();

    let slug = slugify(&input);

    println!("{}", slug);
}

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn stats(text: &str) -> [u32; 26] {
    let mut counts = [0; 26];
    
    for c in text.chars() {
        if c < 'a' || c > 'z' {
            continue;
        }
        let index: usize  = (c as u8 - b'a').into();

        counts[index] += 1;
        // println!("{}", c as u8 - 97);
         
    }
    return counts;
}

fn is_pangram(counts: &[u32]) -> bool {
    
    if counts.len() != 26 {
        return false;
    }

    
    for n in counts{
        if *n == 0 {return false;}    
    }

    true
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() == 0 {
        println!("No arguments provided.");
        return;
    }
    else if args.len() > 1 {
        println!("Too many arguments provided.");
        return;  
    }

    let file = match File::open(&args[0]){
        Ok(f) => f,
        
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
   
    let mut contents = String::new();

    for riga_result in reader.lines(){

        match riga_result {
            Ok(riga) => contents.push_str(&riga),
            Err(e) => {
                println!("Error reading line: {}", e);
                return;
            }   
        }
    }

    contents = contents.to_ascii_lowercase();

    let counts = stats(&contents);

    //println!("{:?}", counts);
    let result = is_pangram(&counts);

    println!("\"{}\" {} a pangram ", contents,  if result {"is"} else {"is not"});



    for (i, num) in counts.iter().enumerate() {
        println!("{}: {}", (b'a' + i as u8) as char, num);
    }







}


// please note, code has been splitted in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{   
    // tests are separated modules, yuou must import the code you are testing
    use super::*;
    
    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 1;
        counts[1] = 1;
        assert!(!is_pangram(&counts));
    }
    
    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }    
    
    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test] 
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
            
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}


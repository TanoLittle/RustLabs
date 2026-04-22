mod table;
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

trait MySlug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

impl<T> MySlug for T
where 
    T: AsRef<str>
{
    fn is_slug(&self) -> bool{
        // The .as_ref() method is used to convert the type T into a reference to a string slice (&str). 
        // This allows us to call the slugify function 
        slugify(self.as_ref()) == self.as_ref()
    }

    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}

fn main() {
    let s1 = String::from ("Hello String") ;
    let s2 = "hello-slice";
    println !("{}", s1.is_slug () ) ; // false
    println !("{}", s2.is_slug () ) ; // true
    let s3 : String = s1.to_slug () ;
    let s4 : String = s2.to_slug () ;
    println !("s3 :{} s4 :{} ", s3 , s4 ) ; // s3:hello - string s4:hello - slice
}

use std::fmt;

/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */


#[allow(dead_code)]
#[derive(Debug)]
pub enum Type {
    INT(i32), // TODO change size of data type... i32 for now
    FLOAT(f32),
    STRING(String),
}


impl Type {
    fn new(st: &str)  -> Type {
        match (st.parse::<i32>(), st.parse::<f32>()) {
            (Ok(n), _) => Type::INT(n),
            (_, Ok(n)) => Type::FLOAT(n),
            (_, _) => Type::STRING(String::from(st))
        }
    }    
}
impl fmt::Display for Type {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.to_string())
    }
}


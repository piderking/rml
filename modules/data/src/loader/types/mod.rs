/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */

pub enum Type {
    INT(i32),
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


use std::collections::HashMap;
use std::fmt::Debug;
use std::{
    any::type_name,
    fmt::{self, Display, format},
    rc::Rc,
};

/**
 * @file mod.rs
 * @description
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */

pub struct Loader {
    cols: Vec<String>,
    data: Vec<Array<Type>>,
}
impl Loader {
    fn new() -> Loader {
        Loader {
            cols: Vec::new(),
            data: Vec::new(),
        }
    }
}




pub enum Array {
    // must be i32, f32, usize, str
    // Distributes refrences to this data...
    INT(Rc<[i32]>),
    FLOAT(Rc<[f32]>),
    STRING(Rc<Box<[String]>>),
    Empty,
}

impl<T: Iterator> From<T> for Array {
    fn from(value: T) -> Self {
        value.map(|i|)
        Self::Empty
    }
}

impl Array {
    fn new() -> Array {
        Array::Empty
    }
}

use std::rc::Rc;

/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */



pub trait Entry {
    
}

pub enum Type <T: Entry> {
    Col( Box<[T]> ),
    Row( Box<[T]> ),
}



impl<T> Iterator for Type<T> where T : Entry {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }
}


pub struct DataLoader < T: Entry > (
    Vec<Rc<Type<T>>>
);
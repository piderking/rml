
/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */


use std::rc::Rc;
mod types;
mod entry;

use parser::File;





pub enum Type <T: entry::Entry> {
    Col( Box<[T]> ),
    Row( Box<[T]> ),
}



pub struct DataLoader < T: entry::Entry>  (
    Vec<Rc<Type<T>>>
);

impl <T> DataLoader<T> where T : entry::Entry {
    pub fn new() -> DataLoader<T>{
        todo!()
    }
}
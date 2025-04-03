
/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */


use std::rc::Rc;

use data::Loader;
use file::File;

mod entry;

mod file;

mod data;

impl File {
    
    /// Generates a loader object from the specified file
    pub fn generate(&self) -> Loader{
        Loader {}
    }
}
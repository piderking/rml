
/**
 * @file mod.rs
 * @description 
 * @author piderking
 * @copyright Copyright (c) 2025 piderking
 */


use std::rc::Rc;

use file::File;
use types::Loader;
mod entry;

mod file;


mod types;

impl File {
    
    /// Generates a loader object from the specified file
    pub fn generate(&self) -> Loader{
        todo!()
    }
} 
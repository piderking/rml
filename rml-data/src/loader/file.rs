use core::panic;
use std::fs::File as FS;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;



pub enum FileExtension {
    CSV,
}

pub struct File<> {
    location: PathBuf, 
    ext: FileExtension
}

impl File {
    pub fn new (ext: &str) -> File{
        
        // determine extension
        let path = Path::new(ext);

        let ex  = match path.extension().and_then(OsStr::to_str).expect("Invalid Extension") {
            "csv" => FileExtension::CSV,
            _ => panic!("Ex=")
            
        };

        File {
            location: path.to_path_buf(),
            ext: ex
        }

    }
    pub fn read(&self) -> FS{
        let file = FS::open(self.location.as_path());    

        file.unwrap()  // should be valid because new created the file
    }
}
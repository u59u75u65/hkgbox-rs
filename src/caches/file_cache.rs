use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{Read, Write};

use caches::common::*;

pub struct FileCache {}

impl FileCache {
    pub fn new() -> FileCache {
        FileCache { }
    }
}

impl Cache for FileCache {
    fn read<P: AsRef<Path>, S: AsRef<Path>>(&self,
                                            cache_path: P,
                                            file_name: S)
                                            -> Result<String, String> {

        let file_path = cache_path.as_ref().join(file_name);
        let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
        Ok(contents)

    }
    fn write<P: AsRef<Path>, S: AsRef<Path>>(&mut self,
                                             cache_path: P,
                                             file_name: S,
                                             s: String)
                                             -> Result<(), String> {

        let file_path = cache_path.as_ref().join(file_name);
        try!(fs::create_dir_all(&cache_path).map_err(|e| e.to_string()));
        let mut file = try!(File::create(file_path).map_err(|e| e.to_string()));
        try!(file.write_all(&s.into_bytes()).map_err(|e| e.to_string()));
        Ok(())

    }
}

use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{Read, Write};

use crate::caches::common::*;

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
                                            -> Result<Vec<u8>, String> {

        let file_path = cache_path.as_ref().join(file_name);
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)

    }
    fn write<P: AsRef<Path>, S: AsRef<Path>>(&mut self,
                                             cache_path: P,
                                             file_name: S,
                                             s: Vec<u8>)
                                             -> Result<(), String> {

        let file_path = cache_path.as_ref().join(file_name);
        fs::create_dir_all(&cache_path).map_err(|e| e.to_string())?;
        let mut file = File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(s.as_slice()).map_err(|e| e.to_string())?;
        Ok(())

    }
}

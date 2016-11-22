use std::path::Path;
use std::fs::File;
use std::fs;

pub trait Cache {
    fn read<P: AsRef<Path>, S: AsRef<Path>>(&self, cache_path: P,
                                                  file_name: S)
                                                  -> Result<Vec<u8>, String> where Self:Sized;
    fn write<P: AsRef<Path>, S: AsRef<Path>>(&mut self, cache_path: P,
                                                   file_name: S,
                                                   s: Vec<u8>)
                                                   -> Result<(), String> where Self:Sized;
}

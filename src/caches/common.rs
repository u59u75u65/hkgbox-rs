use std::path::Path;
use std::fs::File;
use std::fs;

pub trait Cache {
    fn read<P: AsRef<Path>, S: AsRef<Path>>(&self, cache_path: P,
                                                  file_name: S)
                                                  -> Result<String, String>;
    fn write<P: AsRef<Path>, S: AsRef<Path>>(&mut self, cache_path: P,
                                                   file_name: S,
                                                   s: String)
                                                   -> Result<(), String>;
}

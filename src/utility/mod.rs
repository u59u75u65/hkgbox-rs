pub mod string;

// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn readfile(p: String) -> String
{
    let path = Path::new(&p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       Error::description(&why)),
            Ok(file) => file,
        };

    // Read the file contents into a string, returns `io::Result<usize>`
   let mut s = String::new();
   match file.read_to_string(&mut s) {
       Err(why) => panic!("couldn't read {}: {}", display,
                                                  Error::description(&why)),
       Ok(_) => return s,
   }
}

extern crate hkg;

use hkg::utility::client::download;
use hkg::utility::cache::readfile;

fn main(){

    // let body = download();
    // println!("Response: {}", body);
    let s = readfile("demo.html".to_string());
    print!("contains:\n{}", s)
}

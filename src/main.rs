extern crate hkg;

use hkg::utility::client::download;

fn main(){

    let body = download();
    println!("Response: {}", body);

}

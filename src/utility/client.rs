extern crate hyper;
use std::io::Read;
use self::hyper::Client;
use self::hyper::header::Connection;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::cell::Cell;
use std::sync::Arc;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::ops::FnMut;

// pub fn download() -> String
// {
//         // Create a client.
//         let client = Client::new();
//
//         // Creating an outgoing request.
//         let mut res = client.get("http://forum1.hkgolden.com/topics.aspx?type=BW")
//             // set a header
//             .header(Connection::close())
//             // let 'er go!
//             .send().unwrap();
//
//         // Read the Response.
//         let mut body = String::new();
//         res.read_to_string(&mut body).unwrap();
//
//     return body;
// }

pub struct WebResource {
     pub pages: HashMap<String, String>,
     pub client: Client
}

impl WebResource {

    pub fn new() -> Self {
        WebResource {
            pages: HashMap::new(),
            client: Client::new()
        }
    }

    // let fetch_page = move |url: &str| -> String {
    //     download_map.entry(String::from(url))
    //                 .or_insert_with(move || {
    //                     match download_page(&client, &String::from(url)) {
    //                         Ok(s) => s,
    //                         Err(e) => format!("{:?}", e),
    //                     }
    //                 })
    //                 .clone()
    // };
    //
    
    pub fn download_page(&mut self, url: &String) -> Result<String, Error> {
        match self.client.get(url).send() {
            Ok(mut resp) => {
                let mut s = String::new();
                match resp.read_to_string(&mut s) {
                    Ok(size) => Ok(s),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }

    pub fn save(&mut self, url: &str) {
        if !self.pages.contains_key(url) {
            let res = match self.download_page(&String::from(url)) {
                Ok(s) => s,
                Err(e) => format!("{:?}", e),
            };
            self.pages.insert(String::from(url), res);
        }
    }

    pub fn get(&mut self, url: &str) -> String {
        match self.pages.get(url) {
            Some(page) => page.clone(),
            None => { String::from("None") }
        }
    }

    pub fn fetch(&mut self, url: &str) -> String {
        self.save(url);
        self.get(url)
    }
}

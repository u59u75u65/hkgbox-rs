use log::{info, error};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub struct WebResource {
     pub pages: HashMap<String, String>,
}

impl WebResource {

    pub fn new() -> Self {
        WebResource {
            pages: HashMap::new(),
        }
    }

    pub fn fetch(&mut self, url: &String) -> Result<String, Error> {
        info!("web resource #fetch");
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let response = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36")
            .send()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        let s = response.text().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(s)
    }

    pub fn fetch_safe(&mut self, url: &str) -> String {
        match self.fetch(&String::from(url)) {
            Ok(s) => s,
            Err(e) => format!("{:?}", e),
        }
    }

    pub fn find(&mut self, url: &str) -> String {
        match self.pages.get(url) {
            Some(page) => page.clone(),
            None => { String::from("None") }
        }
    }

    pub fn get(&mut self, url: &str) -> String {
        if !self.pages.contains_key(url) {
            let res = self.fetch_safe(url);
            self.pages.insert(String::from(url), res);
        }
        self.find(url)
    }
}

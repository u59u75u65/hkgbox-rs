extern crate hyper;
use std::io::Read;
use self::hyper::Client;
use self::hyper::header::Connection;

pub fn download() -> String
{
        // Create a client.
        let client = Client::new();

        // Creating an outgoing request.
        let mut res = client.get("http://forum1.hkgolden.com/topics.aspx?type=BW")
            // set a header
            .header(Connection::close())
            // let 'er go!
            .send().unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

    return body;
}

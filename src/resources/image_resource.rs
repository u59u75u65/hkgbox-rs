extern crate cancellation;
use self::cancellation::CancellationTokenSource;
extern crate time;
use net::*;
use net::web_resource::*;
use resources::common::*;
use caches::common::*;
use caches::file_cache::*;

extern crate rustc_serialize;
use rustc_serialize::base64::{self, ToBase64};

extern crate hyper;
use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};
use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;

pub struct ImageResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource,
    cache: &'a mut Box<T>,
    client: Client
}

impl<'a, T: 'a + Cache> ImageResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource, cache: &'a mut Box<T>) -> Self {
        ImageResource {
            wr: wr,
            ct: ct,
            cache: cache,
            client: Client::new()
        }
    }
}

impl<'a, T: 'a + Cache> Resource for ImageResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        match item.extra.clone() {
            ChannelItemType::Image(extra) => {
                let url = extra.url;
                let url2 = url.clone();
                let img_path = "data/img/";
                let img_file_name = url.into_bytes().as_slice().to_base64(base64::URL_SAFE);
                let (from_cache, result, reason) = match self.cache.read(&img_path, &img_file_name) {
                    Ok(result) => (true, result, "".to_string()),
                    Err(_) => {
                        match self.client.get(&url2).send() {
                            Ok(mut resp) => {
                                    let mut buffer = Vec::new();
                                    resp.read_to_end(&mut buffer);
                                    self.cache.write(&img_path, &img_file_name, buffer.clone());
                                    (false, buffer, "".to_string())
                                }
                            Err(e) => (false, Vec::new(), e.to_string())
                        }
                    }
                };

                let result_item = ChannelItem {
                    extra: ChannelItemType::Image(ChannelImageItem { url: url2, bytes: result }),
                    result: reason,
                };
                result_item
            },
            _ => {
                let result_item = ChannelItem {
                    extra: ChannelItemType::Image(ChannelImageItem { url: "".to_string(), bytes: Vec::new() }),
                    result: "".to_string(),
                };
                result_item
            }
        }
    }
}

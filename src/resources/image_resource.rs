use resources::*;
use resources::common::*;
use caches::common::*;

use rustc_serialize::base64::{self, ToBase64};

use std::io::Read;

use ::hyper::Client;
use ::hyper::net::HttpsConnector;
use ::hyper_native_tls::NativeTlsClient;
use ::hyper::header::{Headers, UserAgent};

pub struct ImageResource<'a, T: 'a + Cache> {
    cache: &'a mut Box<T>,
    client: Client
}

impl<'a, T: 'a + Cache> ImageResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        ImageResource {
            cache: cache,
            client: Client::with_connector(connector)
        }
    }
}

impl<'a, T: 'a + Cache> Resource for ImageResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        match item.extra.clone() {
            Some(o) => {
                match o {
                    ChannelItemType::Image(extra) => {
                        let url = extra.url;
                        let url2 = url.clone();
                        let img_path = "data/cache/img/";
                        let img_file_name = url.into_bytes().as_slice().to_base64(base64::URL_SAFE);

                        info!("image resource - before find in cache. url: {}", url2.clone());
                        let (from_cache, result, reason) = match self.cache.read(&img_path, &img_file_name) {
                            Ok(result) => {
                                info!("image resource - find in cache success. url:  {}", url2.clone());
                                (true, result, "".to_string())
                            }
                            Err(_) => {
                                info!("image resource - find in cache fail. url:  {}", url2.clone());

                                let mut headers = Headers::new();
                                headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36".to_owned()));

                                match self.client.get(&url2).headers(headers).send() {
                                    Ok(mut resp) => {
                                            info!("image resource - http request success url:  {}", url2.clone());
                                            let mut buffer = Vec::new();
                                            resp.read_to_end(&mut buffer).expect("fail to read buffer from the http response");
                                            self.cache.write(&img_path, &img_file_name, buffer.clone()).expect("fail to write cache");
                                            (false, buffer, "".to_string())
                                        }
                                    Err(e) => {
                                        info!("image resource - http request fail url:  {}", url2.clone());
                                        (false, Vec::new(), e.to_string())
                                    }
                                }
                            }
                        };

                        let url3 = url2.clone();
                        info!("image url: {} reason: {}", url3, reason);
                        let result_item = ChannelItem {
                            extra: Some(ChannelItemType::Image(ChannelImageItem { url: url2, bytes: result })),
                            result: reason,
                        };
                        result_item
                    },
                    _ => {
                        ChannelItem {
                            extra: Some(ChannelItemType::Image(Default::default())),
                            result: Default::default(),
                        }
                    }
                }
            }
            None => {
                ChannelItem {
                    extra: Some(ChannelItemType::Image(Default::default())),
                    result: Default::default(),
                }
            }
        }
    }
}

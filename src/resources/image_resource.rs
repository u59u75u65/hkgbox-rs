use resources::*;
use resources::common::*;
use caches::common::*;

use rustc_serialize::base64::{self, ToBase64};

use std::io::Read;

use ::hyper::Client;
use ::hyper::net::HttpsConnector;
use ::hyper_native_tls::NativeTlsClient;
use ::hyper::header::{Headers, UserAgent};

use std::thread;
use std::sync::mpsc::channel;
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};

use std::sync::mpsc::{Receiver, Sender};

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use caches::file_cache::*;

use crossbeam::*;

use std::thread::park_timeout;
use std::time::{Instant, Duration};
use std::sync::RwLock;

pub struct ImageResource<'a, T: 'a + Cache + Send> {
    cache: &'a mut Box<T>,
    client: Client
}

impl<'a, T: 'a + Cache + Send> ImageResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let mut client = Client::with_connector(connector);
        client.set_read_timeout(Some(::std::time::Duration::from_secs(5)));
        client.set_write_timeout(Some(::std::time::Duration::from_secs(5)));

        ImageResource {
            cache: cache,
            client: client
        }
    }
}

impl<'a, T: 'a + Cache + Send> Resource for ImageResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("[image_resource] #fetch {:?}", item.extra.clone());
        match item.extra.clone() {
            Some(o) => {
                match o {
                    ChannelItemType::Image(extra) => {
                        let url = extra.url;
                        let url2 = url.clone();
                        let img_path = "data/cache/img/";
                        let img_file_name = url.into_bytes().as_slice().to_base64(base64::URL_SAFE);

                        info!("image resource - before find in cache. url: {}", url2.clone());
                        let read_result: Option<(bool, Vec<u8>, String)> = match self.cache.read(&img_path, &img_file_name) {
                            Ok(result) => {
                                info!("image resource - find in cache success. url:  {}", url2.clone());
                                Some( (true, result, Default::default()) )
                            }
                            Err(_) => {
                                info!("image resource - find in cache fail. url:  {}", url2.clone());

                                let (tx_req, rx_req) = channel::<Option<(bool, Vec<u8>, String)>>();
                                let tx_req2 = tx_req.clone();

                                let url3 = url2.clone();
                                let url4 = url2.clone();

                                let mut headers = Headers::new();
                                headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36".to_owned()));

                                match self.client.get(&url3).headers(headers).send() {
                                    Ok(mut resp) => {
                                            // is_done.write().unwrap().store(true, Ordering::Relaxed);
                                            // (*working).store(false, Ordering::Relaxed);
                                            info!("image resource - http request success url:  {}", url3.clone());
                                            let mut buffer = Vec::new();
                                            resp.read_to_end(&mut buffer).expect("fail to read buffer from the http response");
                                            self.cache.write(&img_path, &img_file_name, buffer.clone()).expect("fail to write cache");
                                            (None)
                                        }
                                    Err(e) => {
                                        // is_done.write().unwrap().store(true, Ordering::Relaxed);
                                        // (*working).store(false, Ordering::Relaxed);
                                        info!("image resource - http request fail url:  {}", url3.clone());
                                        Some( (false, Vec::new(), e.to_string()) )
                                    }
                                }
                            }
                        };

                        match read_result {
                            Some((from_cache, result, reason)) => {
                                let url5 = url2.clone();
                                info!("image url: {} reason: {}", url5, reason);
                                let result_item = ChannelItem {
                                    extra: Some(ChannelItemType::Image(ChannelImageItem { url: url2, bytes: result, from_cache: from_cache })),
                                    result: reason,
                                };
                                result_item
                            },
                            None => {
                                ChannelItem {
                                    extra: Some(ChannelItemType::Image(Default::default())),
                                    result: Default::default(),
                                }
                            }
                        }
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

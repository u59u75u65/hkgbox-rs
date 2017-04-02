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

pub struct ImageResource<'a, T: 'a + Cache + Send> {
    cache: &'a mut Box<T>,
    client: Client
}

impl<'a, T: 'a + Cache + Send> ImageResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        ImageResource {
            cache: cache,
            client: Client::with_connector(connector)
        }
    }
}

impl<'a, T: 'a + Cache + Send> Resource for ImageResource<'a, T> {
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


                                let (tx_req, rx_req) = channel::<Option<(bool, Vec<u8>, String)>>();
                                let tx_req2 = tx_req.clone();

                                let url3 = url2.clone();
                                let url4 = url2.clone();
                                let ssl = NativeTlsClient::new().unwrap();
                                let connector = HttpsConnector::new(ssl);
                                let mut headers = Headers::new();
                                headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36".to_owned()));

                                let beginning_park = Instant::now();
                                let timeout = Duration::from_secs(2);

                                let mut is_done = Arc::new(AtomicBool::new(false));
                                let mut is_done2 = is_done.clone();
                                ::crossbeam::scope(|scope| {
                                    scope.spawn(move ||{
                                        let th = thread::current();

                                        let client = Client::with_connector(connector);
                                        match client.get(&url3).headers(headers).send() {
                                            Ok(mut resp) => {
                                                    is_done.store(true, Ordering::Relaxed);
                                                    info!("image resource - http request success url:  {}", url3.clone());
                                                    let mut buffer = Vec::new();
                                                    resp.read_to_end(&mut buffer).expect("fail to read buffer from the http response");
                                                    self.cache.write(&img_path, &img_file_name, buffer.clone()).expect("fail to write cache");
                                                    tx_req.send(Some( (false, buffer, "".to_string()) ) );
                                                }
                                            Err(e) => {
                                                is_done.store(true, Ordering::Relaxed);
                                                info!("image resource - http request fail url:  {}", url3.clone());
                                                tx_req.send( Some( (false, Vec::new(), e.to_string()) ) );
                                            }
                                        }
                                    });
                                    scope.spawn(move || {
                                        while beginning_park.elapsed() < timeout {
                                            let timeout = timeout - beginning_park.elapsed();
                                            park_timeout(timeout);
                                        }
                                        match Arc::try_unwrap(is_done2) {
                                            Ok(o) => {
                                                let is_done_flag = o.load(Ordering::Relaxed);
                                                warn!("is done flag: {} url: {}", is_done_flag, url4.clone());
                                                if !is_done_flag {
                                                    warn!("image request {} is canceled!", url4.clone());
                                                    thread::park_timeout(::std::time::Duration::from_secs(0));
                                                    tx_req2.send(None);
                                                }
                                            }
                                            Err(e) => {
                                                error!("fail to read is done flag, url: {}, reason: {:?}", url4.clone(), e);
                                                tx_req2.send(None);
                                            }
                                        }

                                    })
                                });

                                match rx_req.recv() {
                                    Ok(o) => {
                                        match o {
                                            Some((from_cache, result, reason)) => (from_cache, result, reason),
                                            None => (false, Vec::new(), "".to_string())
                                        }
                                    }
                                    Err(e) => { (false, Vec::new(), e.to_string()) }
                                }
                            }
                        };

                        let url5 = url2.clone();
                        info!("image url: {} reason: {}", url5, reason);
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

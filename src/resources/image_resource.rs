use log::{info, error};
use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;

use base64::{Engine as _, engine::general_purpose};

use std::io::Read;

use std::thread;
use std::sync::mpsc::channel;

use std::sync::mpsc::{Receiver, Sender};

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::caches::file_cache::*;

use crossbeam::*;

use std::thread::park_timeout;
use std::time::{Instant, Duration};
use std::sync::RwLock;

pub struct ImageResource<'a, T: 'a + Cache + Send> {
    cache: &'a mut Box<T>,
}

impl<'a, T: 'a + Cache + Send> ImageResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        ImageResource {
            cache: cache,
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
                        let img_file_name = general_purpose::URL_SAFE.encode(url.as_bytes());

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

                                let client_result = reqwest::blocking::Client::builder()
                                    .timeout(std::time::Duration::from_secs(5))
                                    .build();

                                match client_result {
                                    Ok(client) => {
                                        match client.get(&url3).send() {
                                            Ok(mut resp) => {
                                                info!("image resource - http request success url:  {}", url3.clone());
                                                let mut buffer = Vec::new();
                                                match resp.copy_to(&mut buffer) {
                                                    Ok(_) => {
                                                        match self.cache.write(&img_path, &img_file_name, buffer.clone()) {
                                                            Ok(_) => None,
                                                            Err(_) => Some((false, Vec::new(), "Failed to write cache".to_string()))
                                                        }
                                                    }
                                                    Err(e) => {
                                                        info!("image resource - failed to read response: {:?}", e);
                                                        Some( (false, Vec::new(), e.to_string()) )
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                info!("image resource - http request fail url:  {}", url3.clone());
                                                Some( (false, Vec::new(), e.to_string()) )
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        info!("image resource - failed to create client: {:?}", e);
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

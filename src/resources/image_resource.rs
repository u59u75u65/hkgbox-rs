use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;

use log::info;
use base64::engine::Engine;

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
        info!("[image_resource] #fetch {:?}", item.extra);
        match &item.extra {
            Some(ChannelItemType::Image(extra)) => {
                let url = &extra.url;
                let img_path = "data/cache/img/";
                let img_file_name = base64::engine::general_purpose::URL_SAFE.encode(url);

                info!("image resource - before find in cache. url: {}", url);
                let read_result: Option<(bool, Vec<u8>, String)> = match self.cache.read(img_path, &img_file_name) {
                    Ok(result) => {
                        info!("image resource - find in cache success. url: {}", url);
                        Some((true, result, Default::default()))
                    }
                    Err(_) => {
                        info!("image resource - find in cache fail. url: {}", url);

                        let client_result = reqwest::blocking::Client::builder()
                            .timeout(std::time::Duration::from_secs(5))
                            .build();

                        match client_result {
                            Ok(client) => {
                                match client.get(url).send() {
                                    Ok(mut resp) => {
                                        info!("image resource - http request success url: {}", url);
                                        let mut buffer = Vec::new();
                                        match resp.copy_to(&mut buffer) {
                                            Ok(_) => {
                                                match self.cache.write(img_path, &img_file_name, buffer.clone()) {
                                                    Ok(_) => None,
                                                    Err(_) => Some((false, Vec::new(), "Failed to write cache".to_string()))
                                                }
                                            }
                                            Err(e) => {
                                                info!("image resource - failed to read response: {:?}", e);
                                                Some((false, Vec::new(), e.to_string()))
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        info!("image resource - http request fail url: {}", url);
                                        Some((false, Vec::new(), e.to_string()))
                                    }
                                }
                            }
                            Err(e) => {
                                info!("image resource - failed to create client: {:?}", e);
                                Some((false, Vec::new(), e.to_string()))
                            }
                        }
                    }
                };

                match read_result {
                    Some((from_cache, result, reason)) => {
                        info!("image url: {} reason: {}", url, reason);
                        ChannelItem {
                            extra: Some(ChannelItemType::Image(ChannelImageItem {
                                url: url.clone(),
                                bytes: result,
                                from_cache: from_cache
                            })),
                            result: reason,
                        }
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
}

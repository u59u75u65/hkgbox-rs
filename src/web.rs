use log::info;
use std::thread;
use std::sync::mpsc::{Receiver, Sender};

use crate::caches::file_cache::*;
use crate::resources::*;
use crate::resources::common::*;
use crate::resources::index_resource_api::*;
use crate::resources::show_resource_api::*;
use crate::resources::image_resource::*;

use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Requester {}

impl Requester {
    pub fn new(rx_req: Receiver<ChannelItem>, tx_res: Sender<ChannelItem>, working: Arc<AtomicBool>) -> Self {

        // web client - simplified without nested thread spawning
        thread::spawn(move || {

            while (*working).load(Ordering::Relaxed) {
                match rx_req.recv() {
                    Ok(item) => {
                        info!("[requester] #recv");
                        let mut fc = Box::new(FileCache::new());
                        let tx_res2 = tx_res.clone();

                        match item.extra.clone() {
                            Some(o) => {
                                info!("request: {:?}", o);
                                match o {
                                    ChannelItemType::Index(index_item) => {
                                        info!("[requester] creating IndexResource with page {}", index_item.page);
                                        let mut index_resource = IndexResource::new(&mut fc);
                                        index_resource.set_page(index_item.page);
                                        info!("[requester] fetching from IndexResource");
                                        let result = index_resource.fetch(&item);
                                        info!("[requester] sending index response");
                                        let _ = tx_res2.send(result);
                                        info!("[requester] index response sent");
                                    }
                                    ChannelItemType::IndexWithData(_) | ChannelItemType::IndexWithPageData(_, _, _) => {
                                        info!("[requester] creating IndexResource (default page)");
                                        let mut index_resource = IndexResource::new(&mut fc);
                                        info!("[requester] fetching from IndexResource");
                                        let result = index_resource.fetch(&item);
                                        info!("[requester] sending index response");
                                        let _ = tx_res2.send(result);
                                        info!("[requester] index response sent");
                                    }
                                    ChannelItemType::Show(_) | ChannelItemType::ShowWithData(_) => {
                                        info!("[requester] creating ShowResourceApi");
                                        let mut show_resource = ShowResourceApi::new(&mut fc);
                                        info!("[requester] fetching from ShowResourceApi");
                                        let result = show_resource.fetch(&item);
                                        info!("[requester] sending show response");
                                        let _ = tx_res2.send(result);
                                        info!("[requester] show response sent");
                                    }
                                    ChannelItemType::Image(_) => {
                                        let mut image_resource = ImageResource::new(&mut fc);
                                        let _ = tx_res2.send(image_resource.fetch(&item));
                                    }
                                }
                            }
                            None => { let _ = tx_res2.send(Default::default()); }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        Requester { }
    }
}

use log::{info, error};
use std::thread;
use std::sync::mpsc::{Receiver, Sender};

use crate::caches::file_cache::*;
use crate::resources::*;
use crate::resources::common::*;
use crate::resources::index_resource::*;
use crate::resources::show_resource::*;
use crate::resources::image_resource::*;
use crate::resources::web_resource::*;

use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Requester {}

impl Requester {
    pub fn new(rx_req: Receiver<ChannelItem>, tx_res: Sender<ChannelItem>, working: Arc<AtomicBool>) -> Self {

        // web client
        thread::spawn(move || {

            while (*working).load(Ordering::Relaxed) {
                match rx_req.recv() {
                    Ok(item) => {
                        let mut wr = WebResource::new();
                        let mut fc = Box::new(FileCache::new());
                        let tx_res2 = tx_res.clone();
                        thread::spawn(move || {
                            info!("[requester] #recv");
                            match item.extra.clone() {
                                Some(o) => {
                                    info!("request: {:?}", o);
                                    match o {
                                        ChannelItemType::Index(_) => {
                                            let mut index_resource = IndexResource::new(&mut wr, &mut fc);
                                            tx_res2.send(index_resource.fetch(&item)).expect("[web client] fail to send index request");
                                        }
                                        ChannelItemType::Show(_) => {
                                            let mut show_resource = ShowResource::new(&mut wr, &mut fc);
                                            tx_res2.send(show_resource.fetch(&item)).expect("[web client] fail to send show request");
                                        }
                                        ChannelItemType::Image(_) => {
                                            let mut image_resource = ImageResource::new(&mut fc);
                                            tx_res2.send(image_resource.fetch(&item)).expect("[web client] fail to send image request");
                                        }
                                    }
                                }
                                None => { tx_res2.send(Default::default()); }
                            }
                        });
                    }
                    Err(_) => {}
                }
            }
        });

        Requester { }
    }
}

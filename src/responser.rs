use log::{info, error};
use std::sync::mpsc::Sender;
use ::kuchiki::traits::*;

use crate::status::*;
use crate::state_manager::*;
use crate::resources::*;

pub struct Responser {}

impl Responser {

    pub fn new () -> Self { Responser {} }
    pub fn try_recv (&self, app: &mut crate::App) {
        match app.rx_res.try_recv() {
            Ok(item) => {
                info!("respsoner receive item");
                match item.extra {
                    Some(o) => {
                        match o {
                            ChannelItemType::Show(extra) => {
                                let document = ::kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                                let posturl = get_posturl(&extra.postid, extra.page);

                                app.status_bar.append(&app.screen_manager,
                                                      &format!("[{}-{}:ROK][{}]",
                                                               app.show_item.url_query.message,
                                                               app.show_item.page,
                                                               app.state_manager.is_web_request()));

                                match app.show_builder.build(&document, &posturl) {
                                    Ok(item) => {
                                        app.show_item = item;

                                        // get all images links in an array, and send to background download
                                        let maps = app.show_item.replies.iter().flat_map(|reply| {
                                                reply.body.iter().filter(|node| {
                                                    match node {
                                                        crate::reply_model::NodeType::Image(n) => {
                                                            // Check for external images (HTTP/HTTPS URLs)
                                                            n.data.starts_with("http") || n.data.starts_with("https")
                                                        }
                                                        _ => false,
                                                    }
                                                })
                                            })
                                            .collect::<Vec<_>>();

                                        let count = maps.len();
                                        app.status_bar.append(&app.screen_manager,
                                                              &format!("[SIMG:{count}]", count = count));

                                        for node in &maps {
                                            match node {
                                                crate::reply_model::NodeType::Image(n) => {
                                                    let status_message = image_request(&n.data, &mut app.state_manager, &app.tx_req);
                                                    app.status_bar.append(&app.screen_manager, &status_message);
                                                }
                                                _ => {}
                                            }
                                        }

                                        app.show.reset_y();
                                        crate::screen::common::clear_screen();
                                        app.state_manager.update_state(Status::Show);
                                    },
                                    Err(e) => {
                                        error!("show item failed to build. reason: {:?}", e);
                                        app.status_bar.append(&app.screen_manager, &"[SPFAIL]");
                                    }
                                };
                                app.state_manager.set_to_print_screen(true);
                                app.state_manager.set_web_request(false);
                            }
                            ChannelItemType::ShowWithData(show_item) => {
                                // New API mode - data already parsed
                                app.show_item = show_item;

                                app.status_bar.append(&app.screen_manager,
                                                      &format!("[{}-{}:API-ROK][{}]",
                                                               app.show_item.url_query.message,
                                                               app.show_item.page,
                                                               app.state_manager.is_web_request()));

                                // get all images links in an array, and send to background download
                                let maps = app.show_item.replies.iter().flat_map(|reply| {
                                        reply.body.iter().filter(|node| {
                                            match node {
                                                crate::reply_model::NodeType::Image(n) => {
                                                    n.data.starts_with("http") || n.data.starts_with("https")
                                                }
                                                _ => false,
                                            }
                                        })
                                    })
                                    .collect::<Vec<_>>();

                                let count = maps.len();
                                app.status_bar.append(&app.screen_manager,
                                                      &format!("[SIMG:{count}]", count = count));

                                for node in &maps {
                                    match node {
                                        crate::reply_model::NodeType::Image(n) => {
                                            let status_message = image_request(&n.data, &mut app.state_manager, &app.tx_req);
                                            app.status_bar.append(&app.screen_manager, &status_message);
                                        }
                                        _ => {}
                                    }
                                }

                                app.show.reset_y();
                                crate::screen::common::clear_screen();
                                app.state_manager.update_state(Status::Show);
                                app.state_manager.set_to_print_screen(true);
                                app.state_manager.set_web_request(false);
                            }
                            ChannelItemType::Index(_) => {
                                // Old HTML mode - backward compatibility
                                let document = ::kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                                app.list_topic_items.clear();

                                match app.index_builder.build(&document) {
                                    Ok(items) => {
                                        for item in items {
                                            app.list_topic_items.push(item);
                                        }

                                        app.status_bar.append(&app.screen_manager, &format!("[TOPICS:ROK]"));

                                        crate::screen::common::clear_screen();
                                        app.state_manager.update_state(Status::List);
                                    },
                                    Err(e) => {
                                        error!("index item failed to build. reason: {:?}", e);
                                        app.status_bar.append(&app.screen_manager, &"[IPFAIL]");
                                    }
                                }
                                app.state_manager.set_to_print_screen(true);
                                app.state_manager.set_web_request(false);
                            }
                            ChannelItemType::IndexWithPageData(items, page, max_page, channel, channel_title) => {
                                // New API mode - data already parsed with page info and channel title
                                info!("[Responser] Received {} topics for channel '{}' (page {}/{})",
                                      items.len(), channel, page, max_page);
                                info!("[Responser] First 5 topic titles:");
                                for (i, item) in items.iter().enumerate().take(5) {
                                    info!("  [{}] {}", i+1, item.title.text);
                                }
                                if items.len() > 5 {
                                    info!("  ... and {} more topics", items.len() - 5);
                                }

                                app.list_topic_items.clear();
                                for item in items {
                                    app.list_topic_items.push(item);
                                }

                                app.index_page = page;
                                app.index_max_page = max_page;
                                app.index.set_page(page, max_page);

                                // Use channel title from repository
                                app.index.set_channel(channel.clone(), channel_title.clone());
                                app.current_channel = channel;
                                app.current_channel_title = channel_title;

                                app.status_bar.append(&app.screen_manager, &format!("[TOPICS:API-ROK p{}/{}]", page, max_page));

                                crate::screen::common::clear_screen();
                                app.state_manager.update_state(Status::List);
                                app.state_manager.set_to_print_screen(true);
                                app.state_manager.set_web_request(false);
                            }
                            ChannelItemType::IndexWithData(items) => {
                                // New API mode - data already parsed
                                app.list_topic_items.clear();
                                for item in items {
                                    app.list_topic_items.push(item);
                                }

                                app.status_bar.append(&app.screen_manager, &format!("[TOPICS:API-ROK]"));

                                crate::screen::common::clear_screen();
                                app.state_manager.update_state(Status::List);
                                app.state_manager.set_to_print_screen(true);
                                app.state_manager.set_web_request(false);
                            }
                            ChannelItemType::Image(_extra) => {
                                if item.result != "" {
                                    error!("image item failed to build.");
                                    app.status_bar.append(&app.screen_manager,
                                                          &format!("[RIMG:E-{error}]", error = item.result));
                                } else {
                                    app.status_bar.append(&app.screen_manager, "[RIMG:S]");
                                }
                            }
                        }
                    }
                    None => { }
                }
            }
            Err(ref e) if e == &std::sync::mpsc::TryRecvError::Empty => {
                // Channel is empty, this is expected for try_recv in a loop
            }
            Err(e) => {
                error!("Failed to receive response: {}", e);
            }
        }
    }

}

fn get_posturl(postid: &str, page: usize) -> String {
    let base_url = "http://forum1.hkgolden.com/view.aspx";
    let posturl = format!("{base_url}?type=BW&message={postid}&page={page}",
                          base_url = base_url,
                          postid = postid,
                          page = page);
    posturl
}


fn image_request(url: &str, _state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    info!("image_request - url: {}", url);
    let ci = ChannelItem {
        extra: Some(ChannelItemType::Image(ChannelImageItem {
                                  url: url.to_string(),
                                  bytes: Default::default(),
                                  from_cache: Default::default()
                              })),
        result: Default::default(),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

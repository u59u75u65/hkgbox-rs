use std::sync::mpsc::Sender;
use ::kuchiki::traits::*;

use status::*;
use state_manager::*;
use resources::*;

pub struct Responser {}

impl Responser {

    pub fn new () -> Self { Responser {} }
    pub fn try_recv (&self, mut app: &mut ::App) {
        match app.rx_res.try_recv() {
            Ok(item) => {
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
                                                let f = reply.body.iter().filter(|node| {
                                                        let node2 = node.clone();
                                                        match *node2 {
                                                            ::reply_model::NodeType::Image(ref n) => {
                                                                (n.data.starts_with("http") || n.data.starts_with("https")) && n.alt.starts_with("[img]") && n.alt.ends_with("[/img]")
                                                            }
                                                            _ => false,
                                                        }
                                                    }).collect::<Vec<_>>();
                                                f
                                            })
                                            .collect::<Vec<_>>();

                                        let mut count = app.image_request_count_lock.lock().expect("fail to lock image request count");
                                        *count += maps.len();
                                        // let count = maps.len();
                                        app.state_manager.set_bg_request(true);
                                        app.status_bar.append(&app.screen_manager,
                                                              &format!("[SIMG:{count}]", count = *count));

                                        for node in &maps {
                                            let node2 = node.clone();
                                            match *node2 {
                                                ::reply_model::NodeType::Image(ref n) => {
                                                    let status_message = image_request(&n.data, &mut app.state_manager, &app.tx_req);
                                                    app.status_bar.append(&app.screen_manager, &status_message);
                                                }
                                                _ => {}
                                            }
                                        }

                                        app.show.reset_y();
                                        ::screen::common::clear_screen();
                                        app.state_manager.update_state(Status::Show); //state = Status::Show;
                                    },
                                    Err(e) => app.status_bar.append(&app.screen_manager, &"[SPFAIL]")
                                };

                                app.state_manager.set_web_request(false); // is_web_requesting = false;
                            }
                            ChannelItemType::Index(_) => {
                                let document = ::kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                                app.list_topic_items.clear();

                                match app.index_builder.build(&document) {
                                    Ok(items) => {
                                        for item in items {
                                            app.list_topic_items.push(item);
                                        }

                                        app.status_bar.append(&app.screen_manager, &format!("[TOPICS:ROK]"));

                                        ::screen::common::clear_screen();

                                        app.state_manager.update_state(Status::List); // state = Status::List;
                                        app.state_manager.set_web_request(false); // is_web_requesting = false;
                                    },
                                    Err(e) => app.status_bar.append(&app.screen_manager, &"[IPFAIL]")
                                }

                            }
                            ChannelItemType::Image(extra) => {

                                match app.image_request_count_lock.lock() {
                                    Ok(mut count) => {
                                        if *count == 0 {
                                            app.status_bar.append(&app.screen_manager, &format!("[RIMG:CERR]"));
                                        } else {
                                            *count -= 1;
                                            if item.result != "" {
                                                app.status_bar.append(&app.screen_manager,
                                                                      &format!("[RIMG:E-{count}-{error}]",
                                                                               count = *count,
                                                                               error = item.result));
                                            } else {
                                                app.status_bar.append(&app.screen_manager,
                                                                      &format!("[RIMG:S-{count}]", count = *count));
                                            }
                                        }

                                        ::screen::common::clear_screen();

                                        if *count <= 0 {
                                            app.state_manager.set_bg_request(false);
                                            ::screen::common::clear_screen();
                                            app.state_manager.set_web_request(false); // is_web_requesting = false;
                                        }
                                    }
                                    Err(poisoned) => {
                                        app.status_bar.append(&app.screen_manager, &format!("[IMAGES:LOCKERR]"));
                                    }
                                };
                            }
                        }
                    }
                    None => { }
                }
            }
            Err(_) => {}
        }
    }

}

fn get_posturl(postid: &String, page: usize) -> String {
    let base_url = "http://forum1.hkgolden.com/view.aspx";
    let posturl = format!("{base_url}?type=BW&message={postid}&page={page}",
                          base_url = base_url,
                          postid = postid,
                          page = page);
    posturl
}


fn image_request(url: &String, state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: Some(ChannelItemType::Image(ChannelImageItem {
                                  url: url.to_string(),
                                  bytes: Default::default(),
                              })),
        result: Default::default(),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.set_web_request(true); // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

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

                                        let count = maps.len();
                                        app.status_bar.append(&app.screen_manager,
                                                              &format!("[SIMG:{count}]", count = count));

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
                                    Err(e) => {
                                        error!("show item failed to build. reason: {:?}", e);
                                        app.status_bar.append(&app.screen_manager, &"[SPFAIL]");
                                    }
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
                                    Err(e) => {
                                        error!("index item failed to build. reason: {:?}", e);
                                        app.status_bar.append(&app.screen_manager, &"[IPFAIL]");
                                    }
                                }

                            }
                            ChannelItemType::Image(extra) => {
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

    let url2 = url.clone();

    info!("image_request - url: {}", url2);
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

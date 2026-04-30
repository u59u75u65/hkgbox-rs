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
                            ChannelItemType::IndexWithPageData(items, page, max_page, channel) => {
                                // New API mode - data already parsed with page info
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

                                // Get channel title from channel code
                                let channel_title = get_channel_title(&channel);
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

fn get_channel_title(channel_code: &str) -> String {
    match channel_code {
        // HKGolden channels
        "BW" => "吹水台",
        "HT" => "高登熱",
        "NW" => "最　新",
        "CA" => "時事台",
        "ET" => "娛樂台",
        "SP" => "體育台",
        "FN" => "財經台",
        "ST" => "學術台",
        "SY" => "講故台",
        "EP" => "創意台",
        "SN" => "超自然台",
        "CP" => "優惠台",
        "HW" => "硬件台",
        "IN" => "電訊台",
        "SW" => "軟件台",
        "MP" => "手機台",
        "AP" => "Apps台",
        "blockchain" => "Crypto台",
        "AI" => "AI技術台",
        "GM" => "遊戲台",
        "ED" => "飲食台",
        "TR" => "旅遊台",
        "CO" => "潮流台",
        "AN" => "動漫台",
        "TO" => "玩具台",
        "MU" => "音樂台",
        "VI" => "影視台",
        "DC" => "攝影台",
        "TS" => "汽車台",
        "WK" => "上班台",
        "LV" => "感情台",
        "SC" => "校園台",
        "BB" => "親子台",
        "PT" => "寵物台",
        "HL" => "健康台",
        "MB" => "站務台",
        "RA" => "電　台",
        "AC" => "活動台",
        "BS" => "買賣台",
        "JT" => "直播台",
        "AU" => "成人台",
        "OP" => "考古台",
        // LIHKG channels (numeric IDs)
        "1" => "吹水台",
        "999" => "自選台",
        "2" => "熱　門",
        "3" => "最　新",
        "5" => "時事台",
        "33" => "政事台",
        "38" => "World",
        "15" => "財經台",
        "37" => "房屋台",
        "6" => "體育台",
        "7" => "娛樂台",
        "8" => "動漫台",
        "10" => "遊戲台",
        "11" => "影視台",
        "12" => "講故台",
        "4" => "手機台",
        "9" => "Apps台",
        "22" => "硬件台",
        "26" => "軟件台",
        "41" => "電器台",
        "21" => "音樂台",
        "23" => "攝影台",
        "24" => "玩具台",
        "25" => "寵物台",
        "20" => "汽車台",
        "31" => "創意台",
        "30" => "感情台",
        "36" => "健康台",
        "39" => "家庭台",
        "14" => "上班台",
        "16" => "飲食台",
        "17" => "旅遊台",
        "18" => "學術台",
        "19" => "校園台",
        "13" => "潮流台",
        "40" => "美容台",
        "27" => "活動台",
        "28" => "站務台",
        "29" => "成人台",
        "32" => "黑　洞",
        "34" => "直播台",
        "35" => "電訊台",
        _ => channel_code,
    }.to_string()
}

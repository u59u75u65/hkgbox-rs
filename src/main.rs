extern crate hkg;
extern crate rustbox;
extern crate rustc_serialize;

use std::default::Default;

use rustbox::{Color, RustBox, Key};
use rustc_serialize::json;

use hkg::utility::cache;
use hkg::model::TopicItem;

fn main() {

    // GUI init
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let title = String::from("高登");
    let s = cache::readfile(String::from("topics.json"));
    let collection: Vec<TopicItem> = json::decode(&s).unwrap();

    let mut status = String::from("> ");

    let mut selected_topic_index: usize = 0; // no selection by default

    loop {

        let w = rustbox.width();
        let h = rustbox.height();

        let body_height = if h >= 3 {
            h - 3
        } else {
            0
        };
        let body_width = if w >= 2 {
            w - 2
        } else {
            0
        };

        if selected_topic_index > body_height {
            selected_topic_index = body_height;
        }

        hkg::screen::list::print_header(&rustbox, w, &title);

        hkg::screen::list::print_body(&rustbox,
                   body_width,
                   2,
                   body_height,
                   &collection,
                   selected_topic_index);

        let status_width = if w > status.len() {
            w - status.len()
        } else {
            0
        };
        let status_spacing = (0..status_width).map(|_| " ").collect::<Vec<_>>().join("");

        rustbox.print(0,
                      h - 1,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("{status}{status_spacing}",
                               status = status,
                               status_spacing = status_spacing));

        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }

                    Key::Up => {
                        status = format_status(status, w, "U");
                        if selected_topic_index > 1 {
                            selected_topic_index = selected_topic_index - 1;
                        }
                    }
                    Key::Down => {
                        status = format_status(status, w, "D");

                        if selected_topic_index < body_height {
                            selected_topic_index = selected_topic_index + 1;
                        }
                    }

                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}

fn format_status(status: String, w: usize, s: &str) -> String
{
    if status.len() >= w {
        String::from(format!("{}{}", &"> ", s))
    } else {
        String::from(format!("{}{}", &status, s))
    }
}

// fn debug_load_and_print_topics() {
//     let s = cache::readfile(String::from("topics.json"));
//     let collection: Vec<TopicItem> = json::decode(&s).unwrap();
//
//     println!("topics {:?}", collection.len());
//     debug_print_topics(collection);
// }
//
// fn debug_print_topics(collection: Vec<TopicItem>) {
//     for (i, item) in collection.iter().enumerate() {
//
//         println!("item[{}]= {title} {author_name} {last_replied_date} {last_replied_time} \
//                   {reply_count} {rating}",
//                  i,
//                  title = item.titles[0].text,
//                  author_name = item.author.name,
//                  last_replied_date = item.last_replied_date,
//                  last_replied_time = item.last_replied_time,
//                  reply_count = item.reply_count,
//                  rating = item.rating);
//     }
// }

extern crate hkg;
extern crate rustbox;
extern crate rustc_serialize;

use std::default::Default;

use rustbox::{Color, RustBox, Key};
use rustc_serialize::json;
use rustc_serialize::json::Json;

use hkg::utility::cache;
use hkg::model::ListTopicItem;
use hkg::model::ShowItem;

fn main() {

    // GUI init
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let title = String::from("高登");
    let s = cache::readfile(String::from("topics.json"));
    let collection: Vec<ListTopicItem> = json::decode(&s).unwrap();

    let t = cache::readfile(String::from("view.json"));
    let item: ShowItem = json::decode(&t).unwrap();

    let mut status = String::from("> ");

    let mut list = hkg::screen::list::List::new(&rustbox);
    let mut show = hkg::screen::show::Show::new(&rustbox);

    loop {

        show.print(&title, &item);

        // list.print(&title, &collection);

        print_status(&rustbox, &status);

        // show UI
        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }

                    Key::Up => {
                        let w = rustbox.width();
                        status = format_status(status, w, "U");
                        let tmp = list.get_selected_topic();
                        if tmp > 1 {
                            list.select_topic( tmp - 1 );
                        }
                    }
                    Key::Down => {
                        let w = rustbox.width();
                        status = format_status(status, w, "D");
                        let tmp = list.get_selected_topic();
                        if tmp < list.body_height() {
                            list.select_topic( tmp + 1 );
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

fn print_status(rustbox : &rustbox::RustBox, status: &str)
{
    // for status bar only
    let w = rustbox.width();
    let h = rustbox.height();

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

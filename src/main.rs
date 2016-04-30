extern crate hkg;
extern crate rustbox;
extern crate rustc_serialize;
extern crate chrono;

use std::default::Default;

use rustbox::{Color, RustBox, Key};
use rustc_serialize::json;
use rustc_serialize::json::Json;

use chrono::*;

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

        date_operation_example(&rustbox);

        // show.print(&title, &item);

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

fn date_operation_example(rustbox: &rustbox::RustBox){
    let now = Local::now();

    let dt1 = match Local.datetime_from_str("30/4/2016 9:22", "%d/%m/%Y %H:%M") {
        Ok(v) => v,
        Err(e) => Local::now(),
    };

    let dt2 = now.checked_sub(Duration::seconds(46)).unwrap();
    let dt3 = now.checked_sub(Duration::minutes(6)).unwrap();
    let dt4 = now.checked_sub(Duration::days(17)).unwrap();
    let dt5 = now.checked_sub(Duration::weeks(9)).unwrap();

    rustbox.print(0,
                  0,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &format!("{} {} {} {}",
                   duration_format(&(now - dt2)),
                   duration_format(&(now - dt3)),
                   duration_format(&(now - dt4)),
                   duration_format(&(now - dt5))
              ));

}

fn duration_format(duration: &Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days();
    let hours = duration.num_hours();
    let minutes = duration.num_minutes();

    if weeks > 0 {
        format!("{}w", weeks)
    } else if days > 0 {
        format!("{}d", days)
    } else if hours > 0 {
        format!("{}h", hours)
    } else if minutes > 0 {
        format!("{}m", minutes)
    } else {
        String::from("1m")
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

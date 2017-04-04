
// for (jndex, elm) in tr.as_node().select(".repliers_right .ContentGrid").unwrap().enumerate() {
//     let content = elm.as_node().text_contents();
//     let name = &elm.name;
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}][{:<2}]={:?}", index, jndex, content));
// }

// let content = tr.text_contents();
// rustbox.print(1,
//               index + 4,
//               rustbox::RB_NORMAL,
//               Color::White,
//               Color::Black,
//               &format!("[{:<2}]={:?}", index, content));

// for (jndex, div) in tr.as_node().children().enumerate() {
//     let content = div.as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// for (jndex, div) in tr.as_node().select(".repliers_right").unwrap().enumerate() {
//     let content = div.as_node().as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// for (jndex, div) in tr.as_node().select(".repliers_right").unwrap().collect::<Vec<_>>().iter().enumerate() {
//     let content = div.as_node().as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// let c = &tr.as_node().select(".repliers_right .ContentGrid").unwrap().collect::<Vec<_>>()[0];
// let content = c.as_node().as_text();
// rustbox.print(1, index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index, content));


// fn date_operation_example(rustbox: &rustbox::RustBox) {
//     let now = Local::now();
//
//     let dt1 = match Local.datetime_from_str("30/4/2016 9:22", "%d/%m/%Y %H:%M") {
//         Ok(v) => v,
//         Err(e) => Local::now(),
//     };
//
//     let dt2 = now.checked_sub(Duration::seconds(46)).unwrap();
//     let dt3 = now.checked_sub(Duration::minutes(6)).unwrap();
//     let dt4 = now.checked_sub(Duration::days(17)).unwrap();
//     let dt5 = now.checked_sub(Duration::weeks(9)).unwrap();
//
//     rustbox.print(0,
//                   0,
//                   rustbox::RB_BOLD,
//                   Color::White,
//                   Color::Black,
//                   &format!("{} {} {} {}",
//                    duration_format(&(now - dt2)),
//                    duration_format(&(now - dt3)),
//                    duration_format(&(now - dt4)),
//                    duration_format(&(now - dt5))
//               ));
//
// }

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

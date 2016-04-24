extern crate hkg;

extern crate kuchiki;
use kuchiki::traits::*;

use hkg::utility::client::download;
use hkg::utility::cache::readfile;

fn main(){

    // let body = download();
    // println!("Response: {}", body);

    // let html = readfile("demo.html".to_string());
    // print!("contains:\n{}", html)

    let path = "demo.html";

    let document = kuchiki::parse_html().from_utf8().from_file(&path).unwrap();
    // println!("{:?}", document.to_string());

    let topic_list_panel = document.select(".Topic_ListPanel").unwrap().collect::<Vec<_>>();

    if topic_list_panel.len() >  0 {
        let trs = topic_list_panel[0].as_node().select("tr").unwrap().collect::<Vec<_>>();
        println!("trs len: {:?}", trs.len());

        for (index, tr) in trs.iter().enumerate() {
            let tr_node = tr.as_node();
            // println!("{:?}", tr_node.text_contents());

            let tds = tr_node.select("td").unwrap().collect::<Vec<_>>();
            println!("tds[{:?}]={:?}", index, tds.len());
        }
    }
    else
    {
        println!("not found");
    }

}

mod manipulation;
mod util;
mod constants;
mod online;
mod data_formats;
mod page_generation;
mod fixes;
mod link_gather;

pub use {constants::*, manipulation::*, online::*, util::*, data_formats::{WordGender::*, *}};
use std::fs;

#[tokio::main]
async fn main() -> () {
    let start_time = util::sys_time();
    let client = reqwest::Client::new();

    //fix if page exists in different language
    let c = vec![];

    for i in &c {
        let _ = online::upload_wrd(&client, *i).await;
    }

    let txt = operations(&client, "pan").await;
    // let txt = operations(&client, "bzdura").await;
    // let txt = operations(&client, "anielskie włosy").await;
    // let txt = operations(&client, "talent").await;

    let end_time = util::sys_time();
    println!("Elapsed time: {} seconds", util::ns_to_s(end_time - start_time));
    println!("Words processed: {}", &c.len());

    let v = &fs::read_to_string("word_list.txt").expect("err1");
    let samples: Samples = serde_json::from_str(v).expect("err2");

    println!("|:{}", samples.nouns.len());
}
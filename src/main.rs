mod manipulation;
mod util;
mod constants;
mod online;
mod data_formats;
mod page_generation;
mod fixes;
mod link_gather;
mod bookkeeper;

pub use {online::*, util::*, data_formats::{WordGender::*, *}, bookkeeper::get_keeper};

#[tokio::main]
async fn main() -> () {
    let start_time = sys_time();
    let _client = reqwest::Client::new();

    //fix if page exists in different language
    let add_list = vec![""];
    for _word in &add_list {
        // let _ = upload_wrd(&client, *word).await;
    }

    // let txt = operations(&client, "pan").await;
    // let txt = operations(&client, "bzdura").await;
    // let txt = operations(&client, "anielskie włosy").await;
    // let txt = operations(&client, "talent").await;

    let end_time = sys_time();
    println!("Elapsed time: {} seconds", ns_to_s(end_time - start_time));
    println!("Words processed: {}", &add_list.len());

    //Bookkeeper functionality
    if add_list.len() > 0 {
        bookkeeper::increment_count(add_list.len() as i32);
        bookkeeper::update_ts(end_time);
        bookkeeper::append_list(add_list);
        println!("Updated Total: {}", bookkeeper::get_count());
        println!("Last Updated: {}", bookkeeper::time());
    }
}
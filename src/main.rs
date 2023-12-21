mod manipulation;
mod util;
mod constants;
mod online;
mod data_formats;
mod page_generation;
mod fixes;

pub use {constants::*, manipulation::*, online::*, util::*, data_formats::{WordGender::*, *}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let start_time = util::sys_time();
    let client = reqwest::Client::new();

    //fix if page exists in different language
    //fix em
    let c = vec!["przeszłość","teraźniejszość","przeszłość", "prymicja"];

    for i in &c {
        let _ = online::upload_wrd(&client, *i).await;
    }
    let end_time = util::sys_time();
    println!("Elapsed time: {} seconds", util::ns_to_s(end_time - start_time));
    println!("Words processed: {}", &c.len());
    Ok(())
}
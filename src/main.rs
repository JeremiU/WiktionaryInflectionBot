mod manipulation;
mod util;
mod constants;
mod online;
mod data_formats;
mod page_generation;
mod fixes;

pub use manipulation::*;
pub use util::*;
pub use constants::*;
pub use online::*;
pub use data_formats::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let c = vec!["zupka"];

    for i in c {
        // let _ = online::upload_wrd(i).await;
    }
    let client = reqwest::Client::new();

    let i = fixes::get_wrd_sect(&client, "psa").await;

    println!("{}", i);
    //proper noun support MISSING! 
    Ok(())
}
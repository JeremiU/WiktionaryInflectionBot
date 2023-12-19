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
pub use data_formats::WordGender::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    //fix if page exists in different language
    let c = vec!["szmateks"];

    for i in c {
        let _ = online::upload_wrd(&client, i).await;
    }
    Ok(())
}
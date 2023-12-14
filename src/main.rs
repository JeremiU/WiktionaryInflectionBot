mod manipulation;
mod util;
mod constants;
mod online;

pub use manipulation::*;
pub use util::*;
pub use constants::*;
pub use online::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let c = vec!["adornacja","ozdoba"];

    for i in c {
        let _ = online::upload_wrd(i).await;
    }
    //proper noun support MISSING! 
    Ok(())
}
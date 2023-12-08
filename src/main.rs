mod manipulation;
mod util;
mod constants;
mod online;

//pub use manipulation::*;
//pub use util::*;
//pub use constants::*;
use reqwest;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

use urlencoding::encode;


#[derive(Debug, Deserialize)]
struct WebData {
    api_url: String,
    api_tkn: String,
    app_key: String,
    app_sec: String,
    acc_tok: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let file_path = "WebData.json";

    // Read the JSON file
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let web_data: WebData = serde_json::from_value(serde_json::from_str(&contents).expect("Err 1")).expect("Err 2");

    // Make an authenticated request using the obtained access token
    make_authenticated_request(&web_data).await?;

    Ok(())
}

async fn make_authenticated_request(web_data: &WebData) -> Result<(), reqwest::Error> {

    // Create a client with default settings
    let client = reqwest::Client::new();
    
    // Set your Wiktionary username and password
    let username = "";
    let password = "";

   
    // Step 2
    let mut login_params = HashMap::new();
    login_params.insert("action", "login");
    login_params.insert("lgname", username);
    login_params.insert("lgpassword", password);
    login_params.insert("lgtoken", &web_data.acc_tok);
    login_params.insert("format", "json");

    let response = client
    .post(&web_data.api_url)
    .form(&login_params)
    .send()
    .await?;

    // Parse and print the login response
    let body = response.text().await?;
    println!("Login response: {}", body);


    let txt = "==Polish==

===Pronunciation===
{{pl-p}}

===Noun===
{{head|pl|noun form}}

# {{inflection of|pl|kolorwanka||ins|s}}";

    let mut q_params = HashMap::new();
        q_params.insert("action", "edit");
        q_params.insert("action", "edit");
        q_params.insert("format", "json");
        q_params.insert("title", "kolorowanką");
        q_params.insert("text", &txt);
        q_params.insert("summary", "Added inflection page");
        q_params.insert("tags","");
        q_params.insert("bot", "1");
        q_params.insert("createonly", "1");
        q_params.insert("contentmodel","wikitext");
        q_params.insert("token", &web_data.acc_tok);
        q_params.insert("formatversion", "2");
    
    let response = client
        .post(&web_data.api_url)
        .form(&q_params)
        .send()
        .await?;


    // Process the response as needed
    println!("Response Status: {:?}", response.status());
    println!("\n\n\nResponse: {:?}", response.text().await?);
    println!("Token: {}", &token)
    ;
//    println!("\n\n\nFull URL: {}", &encoded_string); 

    Ok(())
}

// Continue with the rest of your code
//fn main() {
//    process("okno");
//    process("koń");
//    process("kwadrat");
//    process("nożyk");
//    process("bezwzględny");
//    process("pierdolić");
//    process("chodzić"); //impf-2/3
  //  process("srać"); //impf 3/3
//    process("dosrać"); //pf
//    process("migać");
//    process("pośredniak");
//    process("kolorowanka");
//    process("malowanka");
//    process("kraszanka");
//    process("pierdolony");
//    process("większość_bezwzględna"); -> debug
//    process("większość_absolutna"); -> debug
//obiad
//konitrut
//wstać
//pomóc
//gniewać
//	online::main();
//}
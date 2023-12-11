mod manipulation;
mod util;
mod constants;
mod online;

//pub use manipulation::*;
//pub use util::*;
//pub use constants::*;
use reqwest;
//use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct WebData {
    api_url: String,
    api_tkn: String,
    app_key: String,
    app_sec: String,
    acc_tok: String
}

fn data() -> WebData {
    let file_path = "WebData.json";

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    return serde_json::from_value(serde_json::from_str(&contents).expect("Err 1")).expect("Err 2");
}

fn extract_csrf(json_str: &str) -> Option<String> {
    let parsed: Result<Value, _> = serde_json::from_str(json_str);

    if let Ok(Value::Object(obj)) = parsed {
        if let Some(query) = obj.get("query") {
            if let Some(tokens) = query.get("tokens") {
                if let Some(csrftoken) = tokens.get("csrftoken") {
                    if let Some(csrf) = csrftoken.as_str() {
                        return Some(csrf.to_string());
                    }
                }
            }
        }
    }
    None
}

async fn csrf_token(web_data: &WebData) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("action", "query");
    params.insert("meta", "tokens");
    params.insert("format", "json");
    params.insert("formatversion", "2");

    let response = client.post("https://en.wiktionary.org/w/api.php")
    .header("Authorization", format!("Bearer {}", web_data.acc_tok))
    .form(&params)
    .send()
    .await?;

    println!("Status: {}" , response.status());

    let bd = response.text().await?;
    Ok(extract_csrf(&bd).unwrap())
}

async fn edit_wiki_page(web_data: &WebData, csrf_token: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let txt = 
"==Polish==

===Pronunciation===
{{pl-p}}
    
===Noun===
{{head|pl|noun form}}
    
# {{inflection of|pl|kolorowanka||voc|s}}";

    let mut params = HashMap::new();
    params.insert("action", "edit");
    params.insert("title", "kolorowanko");
    params.insert("appendtext", &txt);
    params.insert("summary", "Added inflection page");
    params.insert("tags","");
    params.insert("bot", "1");
    params.insert("createonly", "1"); //will not update if exists
    params.insert("contentmodel","wikitext");
    params.insert("token", &csrf_token);
    params.insert("formatversion", "2");

    let response = client.post("https://en.wiktionary.org/w/api.php")
        .header("Authorization", format!("Bearer {}", web_data.acc_tok))
        .form(&params)
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let web_data = data();

    let csrf_token = csrf_token(&web_data).await.expect("NO CSRF");

    edit_wiki_page(&web_data, &csrf_token).await;

    Ok(())
}
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use crate::manipulation;

#[derive(Debug, Deserialize)]
struct WebData {
    api_url: String,
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

async fn csrf_token(client: &reqwest::Client, web_data: &WebData) -> Result<String, reqwest::Error> {
    let response = client.post(&web_data.api_url)
    .header("Authorization", format!("Bearer {}", &web_data.acc_tok))
    .form(&[("action", "query"), ("meta", "tokens"), ("format", "json"), ("formatversion", "2")])
    .send()
    .await?;

    let bd = response.text().await?;
    Ok(extract_csrf(&bd).unwrap())
}

async fn edit_wiki_page(client: &reqwest::Client, infl_wrd: &str, txt: &str, web_data: &WebData, csrf_token: &str) -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("action", "edit");
    params.insert("title", &infl_wrd);
    params.insert("appendtext", &txt);
    params.insert("summary", "Added inflection page");
    params.insert("tags","");
    params.insert("bot", "1");
//    params.insert("createonly", "1"); //will not update if exists
    params.insert("contentmodel","wikitext");
    params.insert("token", &csrf_token);
    params.insert("formatversion", "2");
    
    let response = client.post(&web_data.api_url)
        .header("Authorization", format!("Bearer {}", web_data.acc_tok))
        .form(&params)
        .send()
        .await?;

    println!("Status: {}", response.status());
    
    Ok(())
}

pub async fn upload_wrd(wrd: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let web_data = data();
    let csrf_token = csrf_token(&client, &web_data).await.expect("NO CSRF");

    let wrd_data = manipulation::process(&client, &wrd).await;
    for i in 0..wrd_data.pages.len() {
        let _ = edit_wiki_page(&client, wrd_data.pages[i].0.as_str(), wrd_data.pages[i].1.as_str(), &web_data, &csrf_token).await;

        println!("Page: {:?}", wrd_data.inflected_words[i]);
    }
    Ok(())
}
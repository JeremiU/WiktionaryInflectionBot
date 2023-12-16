use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

use crate::{manipulation, raw_html, WikiContent};

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
    let body = make_call(client, &[("action", "query"), ("meta", "tokens")], web_data).await.unwrap();
    Ok(extract_csrf(&body).unwrap())
}

async fn edit_wiki_page(client: &reqwest::Client, infl_wrd: &str, txt: &str, web_data: &WebData, csrf_token: &str) -> Result<(), reqwest::Error> {
    let params = &[("action", "edit"), ("title", &infl_wrd), 
    ("appendtext", &txt), ("summary", "Added inflection page"), ("tags", ""), ("bot", "1"), 
    ("contentmodel","wikitext"), ("token", &csrf_token)];
    let _ = make_call(client, params, web_data);
    
    Ok(())
}

async fn make_call(client: &reqwest::Client, params: &[(&str, &str)], web_data: &WebData) -> Result<String, reqwest::Error> {
    let mut params = params.to_vec();
    params.extend_from_slice(&[("format", "json"), ("formatversion", "2")]);

    let response = client.post(&web_data.api_url)
    .header("Authorization", format!("Bearer {}", &web_data.acc_tok))
    .form(&params)
    .send()
    .await?;

    let body = response.text().await?;
    return Ok(body);
}

pub async fn upload_wrd(wrd: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let web_data = data();
    let csrf_token = csrf_token(&client, &web_data).await.expect("NO CSRF");

    let wrd_data = manipulation::process(&client, &wrd).await;
    for i in 0..wrd_data.pages.len() {
        let _ = edit_wiki_page(&client, &wrd_data.pages[i].title, &wrd_data.pages[i].body, &web_data, &csrf_token).await;
        println!("Page: {:?}", wrd_data.inflected_words[i]);
    }
    Ok(())
}

pub async fn wikt_text(client: &reqwest::Client, wrd: &str) -> Result<WikiContent, reqwest::Error> {
    let web_data = data();
    
    let params = &[("action","parse"), ("page", &wrd), 
    ("prop", "sections|links|wikitext|text"), ("disablelimitreport", "1"),
    ("preview","1")];

    let str = make_call(&client, params, &web_data).await.expect("Incorrect call!");

    let res: Result<WikiContent, serde_json::Error> = serde_json::from_str(&str);
    Ok(res.expect("msg"))
}
use reqwest::{self, Error, Client};
use serde_json::Value;

use crate::{manipulation::process, WikiContent, WebData, util::client_data, fixes::check_wrd};

fn extract_csrf(json_str: &str) -> Option<String> {
    let parsed: Result<Value, _> = serde_json::from_str(json_str);

    if let Value::Object(obj) = parsed.expect("ERR") {
        let csrftoken = obj.get("query").unwrap().get("tokens").unwrap().get("csrftoken").unwrap(); 
        return Some(csrftoken.as_str().unwrap().to_string());
    }
    None
}

async fn csrf_token(client: &Client, web_data: &WebData) -> Result<String, Error> {
    let body = make_call(client, &[("action", "query"), ("meta", "tokens")], web_data).await.unwrap();
    Ok(extract_csrf(&body).unwrap())
}

async fn edit_wiki_page(client: &Client, infl_wrd: &str, txt: &str, web_data: &WebData, csrf_token: &str) -> () {
    let params = &[("action", "edit"), ("title", &infl_wrd), 
    ("appendtext", &txt), ("summary", "Added inflection page"), ("tags", ""), ("bot", "1"), 
    ("contentmodel","wikitext"), ("token", &csrf_token)];
    let _ = make_call(client, params, web_data).await;
}

async fn make_call(client: &Client, params: &[(&str, &str)], web_data: &WebData) -> Result<String, Error> {
    let mut params = params.to_vec();
    params.extend_from_slice(&[("format", "json"), ("formatversion", "2")]);

    let response = client.post(&web_data.api_url)
    .header("Authorization", format!("Bearer {}", &web_data.acc_tok))
    .form(&params)
    .send()
    .await?;

    return Ok(response.text().await?);
}

pub async fn upload_wrd(client: &Client, wrd: &str) {
    let web_data = client_data();
    let csrf_token = csrf_token(&client, &web_data).await.expect("NO CSRF");

    let wrd_data = process(&client, &wrd).await.expect("wrd");
    println!("word: {}", wrd_data.lemma.word);
    println!("\tgender: {:?}", wrd_data.lemma.gender);
    println!("\tclass: {:?}", wrd_data.lemma.class);
    for i in 0..wrd_data.pages.len() {
        // let _ = edit_wiki_page(&client, &wrd_data.pages[i].title, &wrd_data.pages[i].body, &web_data, &csrf_token).await;
        println!("Page: {:?}", wrd_data.inflected_words[i]);
    }
}

pub async fn is_polish_entry(client: &Client, wrd: &str) -> bool {
    let content = wikt_text(client, wrd).await.unwrap();
    return content.parse.wiki_text.contains("==Polish==");
}

pub async fn wikt_text(client: &Client, wrd: &str) -> Option<WikiContent> {
    let web_data = client_data();
    
    let params = &[("action","parse"), ("page", &wrd), 
    ("prop", "sections|links|wikitext|text"), ("disablelimitreport", "1"),
    ("preview","1")];

    let str = make_call(&client, params, &web_data).await.expect("Incorrect call!");

    let res: Result<WikiContent, serde_json::Error> = serde_json::from_str(&str);

    if res.is_ok() {
        return Some(res.expect("msg"));
    } else {
        return None;
    }
}

pub async fn operations(client: &Client, wrd: &str) {
    let content = wikt_text(client, wrd).await;
    if content.is_none() {
        println!("Entry \'{}\' doesn't exist!", wrd);
    } else {
        println!("Entry \'{}\':", wrd);
        let wrd_data = process(&client, &wrd).await.expect("msg-");
        let is_polish_entry = is_polish_entry(client, wrd).await;
        
        println!("  Is Polish: {}", is_polish_entry);
        if !is_polish_entry {
            return;
        }
        println!("  Pronounciation: {}", wrd_data.pronounciation_base);
        println!("  Class: {}", wrd_data.lemma.class);
        let _ = check_wrd(client, wrd).await;
    }
}
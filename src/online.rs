use reqwest::{self, Error, Client};
use serde_json::{Value, Error as Err, from_str, from_value};

use crate::{manipulation::process, WikiContent, WebData, util::client_data, fixes::check_wrd, link_gather::get_links, WikiSection, SingleSection};

fn extract_csrf(json_str: &str) -> Option<String> {
    let parsed: Result<Value, _> = from_str(json_str);

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
    // println!("DEBUG: txt");
    println!("{}", txt);
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

    // for i in 0..wrd_data.pages.len() {
    //     let _ = edit_wiki_page(&client, &wrd_data.pages[i].title, &wrd_data.pages[i].body, &web_data, &csrf_token).await;
    //     println!("Page: {:?}", wrd_data.inflected_words[i]);
    // }
}

pub async fn is_polish_entry(client: &Client, wrd: &str) -> bool {
    let content = wikt_text(client, wrd).await.unwrap();
    return content.wiki_text.contains("==Polish==");
}

pub async fn wikt_text(client: &Client, wrd: &str) -> Option<WikiContent> {
    let web_data = client_data();

    let params = &[("action","parse"), ("page", &wrd), ("prop","sections"),
    ("disablelimitreport", "1"), ("preview","1")];

    let str = make_call(&client, params, &web_data).await.expect("Incorrect call!");
    let val: Result<WikiSection, Err> = from_str(&str);
    let x: Vec<SingleSection> = val.expect("val?").inner.sections.into_iter().filter(|f| f.title.eq("Polish")).collect();
    if x.len() == 0 {
        println!("No polish entry!");
        return None;
    }
    
    let params = &[("action","parse"), ("page", &wrd), ("section", &x[0].index),
    ("prop", "links|wikitext|text"), ("disablelimitreport", "1"),
    ("preview","1")];

    let json: Value = from_str(&make_call(&client, params, &web_data).await.expect("Incorrect call!")).expect("JSON");
    return from_value(json.get("parse").expect("No parse?").clone()).ok();
}

pub async fn operations(client: &Client, wrd: &str) {
    println!("wrd: {}", wrd);
    let content = wikt_text(client, wrd).await;

    println!("     PL Exists: {}", !content.is_none());
    if !content.is_none() {
        println!("Entry \'{}\':", wrd);
        get_links(content.expect("pg"), wrd);
        // println!("Page links: {:#?}", get_links(content.expect("pg"), wrd));    
        // let wrd_data = process(&client, &wrd).await.expect("msg-");        
        // println!("  Pronounciation: {}", wrd_data.pronounciation_base);
        // println!("  Class: {}", wrd_data.lemma.class);
        // let _ = check_wrd().await;    
    }
}
use reqwest;
use serde_json::Value;
use reqwest::{Error, Client};

use crate::{manipulation, WikiContent, WebData, util::client_data};

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

async fn csrf_token(client: &Client, web_data: &WebData) -> Result<String, Error> {
    let body = make_call(client, &[("action", "query"), ("meta", "tokens")], web_data).await.unwrap();
    Ok(extract_csrf(&body).unwrap())
}

async fn edit_wiki_page(client: &Client, infl_wrd: &str, txt: &str, web_data: &WebData, csrf_token: &str) -> Result<(), Error> {
    let params = &[("action", "edit"), ("title", &infl_wrd), 
    ("appendtext", &txt), ("summary", "Added inflection page"), ("tags", ""), ("bot", "1"), 
    ("contentmodel","wikitext"), ("token", &csrf_token)];
    let _ = make_call(client, params, web_data).await;
    Ok(())
}

async fn make_call(client: &Client, params: &[(&str, &str)], web_data: &WebData) -> Result<String, Error> {
    let mut params = params.to_vec();
    params.extend_from_slice(&[("format", "json"), ("formatversion", "2")]);

    let response = client.post(&web_data.api_url)
    .header("Authorization", format!("Bearer {}", &web_data.acc_tok))
    .form(&params)
    .send()
    .await?;

    let body = response.text().await?;

    println!("call made {:?}", params);
    return Ok(body);
}

pub async fn upload_wrd(client: &Client, wrd: &str) -> Result<(), Error> {
    let web_data = client_data();
    let csrf_token = csrf_token(&client, &web_data).await.expect("NO CSRF");

    let wrd_data = manipulation::process(&client, &wrd).await;
    println!("word: {}", wrd_data.lemma.word);
    println!("\tgender: {:?}", wrd_data.lemma.gender);
    println!("\tclass: {:?}", wrd_data.lemma.class);
    for i in 0..wrd_data.pages.len() {
        let _ = edit_wiki_page(&client, &wrd_data.pages[i].title, &wrd_data.pages[i].body, &web_data, &csrf_token).await;
        println!("Page: {:?}", wrd_data.inflected_words[i]);
    }
    Ok(())
}

pub async fn wikt_text(client: &Client, wrd: &str) -> Result<WikiContent, Error> {
    let web_data = data();
    
    let params = &[("action","parse"), ("page", &wrd), 
    ("prop", "sections|links|wikitext|text"), ("disablelimitreport", "1"),
    ("preview","1")];

    let str = make_call(&client, params, &web_data).await.expect("Incorrect call!");

    let res: Result<WikiContent, serde_json::Error> = serde_json::from_str(&str);
    Ok(res.expect("msg"))
}
use regex::Regex;
use reqwest::Client;

use crate::{find_line, raw_html, str_split, constants::HTML_PL_HEADER, manipulation::process, InflectionData, Word};

//doesn't work with mult. etymology words
pub async fn get_wrd_sect(client: &reqwest::Client, word: &str) -> i8 {
    let raw_html = raw_html(client, word).await;
    let raw_html = str_split(&raw_html, "\n");
    
    let pl_hd: usize = find_line(&raw_html, HTML_PL_HEADER).try_into().unwrap();
    let mut counter = 0;

    for i in 0..raw_html.len() {
        let txt = &raw_html[i];
        if txt.starts_with("<h3>") {
            counter = counter + 1;
            
        }
        if txt.starts_with("<h3>") {
            let pat_wrd = Regex::new(r"Noun</span>").unwrap();
            if let Some(_) = pat_wrd.captures(txt) {
                if i > pl_hd {
                    return counter;
                }               
            }
        }
    }
    return counter;
}

fn check_single_noun(word: &Word) {
    println!("    Checking: {}", word.lemma.word);
    //check pronounciation
    //check gender
    let _ = check_lines(word);
}

fn check_lines(word: &Word) {
    println!("{:?}", word.wiki_data.parse.html_text);
}

pub async fn check_wrd(client: &Client, wrd: &str) {
    let wrd_data = process(&client, &wrd).await.expect("ha");
    for inf in wrd_data.inflected_words {
        let wrd = process(&client, &inf.inflected_word).await;
        println!("    {}", inf.inflected_word);
        if wrd.is_none() {
            println!("    NEED TO CREATE: {}", &inf.inflected_word);
            continue;
        } else  {
            println!("hallo");
            let _ = check_single_noun(&wrd.expect("No-Err"));                
        }
    }
}